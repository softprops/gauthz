//! An interface for fetch Google API tokens targetting
//! [Server to Server](https://developers.google.com/identity/protocols/OAuth2ServiceAccount)
//! applications
//!
//! # examples
//!
//! ```no_run
//! // gauthz interfaces
//! extern crate gauthz;
//! // tokio async io
//! extern crate tokio_core;
//! // futures combinators
//! extern crate futures;
//!
//! use futures::Stream;
//! use gauthz::{Credentials, Scope, Tokens};
//! use tokio_core::reactor::Core;
//! use std::thread::sleep_ms;
//!
//! fn main() {
//!   let mut core = Core::new().unwrap();
//!   let tokens = Tokens::new(
//!      &core.handle(),
//!      Credentials::default().unwrap(),
//!     vec![Scope::CloudPlatform],
//!   );
//!
//!   // warning: this token dispenser never ends!
//!   let dispenser = tokens.stream().for_each(|token| {
//!       println!("{}", token.value());
//!       Ok(sleep_ms(2000)) // do something interesting
//!    });
//!
//!   println!("{:#?}", core.run(dispenser))
//! }
//! ```
//!
//! # Cargo features
//!
//! This crate has one Cargo feature, `tls`, which adds HTTPS support via the `Tokens::new`
//! constructor. This feature is enabled by default.
#![warn(missing_docs)]

#[macro_use]
extern crate serde_derive;
extern crate medallion;
extern crate serde_json;
extern crate time;
#[macro_use]
extern crate error_chain;
extern crate futures;
extern crate hyper;
#[cfg(feature = "tls")]
extern crate hyper_tls;
extern crate tokio_core;

use std::env;
use std::fs::File;
use std::io::Read;
use std::time::{Duration, Instant};

use futures::{Future as StdFuture, Stream as StdStream, future, stream};
use hyper::{Client as HyperClient, Method, Request};
use hyper::client::{Connect, HttpConnector};
use hyper::header::ContentType;
#[cfg(feature = "tls")]
use hyper_tls::HttpsConnector;
use medallion::{Algorithm, Header, Payload, Token};
use tokio_core::reactor::Handle;

pub mod error;
use error::*;
pub use error::{Error, Result};
mod scope;
pub use scope::*;

/// A `Future` with an error type pinned to `gauthz::Error`
pub type Future<T> = Box<StdFuture<Item = T, Error = Error>>;

/// A `Stream` with an error type pinned to `gauthz::Error`
pub type Stream<T> = Box<StdStream<Item = T, Error = Error>>;

const TOKEN_URL: &str = "https://www.googleapis.com/oauth2/v4/token";

/// Authentication credential information generated from google api console
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct Credentials {
    /// a pem encoded rsa key
    private_key: String,
    /// account email
    client_email: String,
}

impl Credentials {
    /// Attempts to resolve credentials from location
    /// defined by common Google API env var
    /// `GOOGLE_APPLICATION_CREDENTIALS`
    pub fn default() -> Result<Credentials> {
        let file = File::open(
            env::var("GOOGLE_APPLICATION_CREDENTIALS").map_err(|_| {
                ErrorKind::Msg("missing GOOGLE_APPLICATION_CREDENTIALS".into())
            })?,
        )?;
        Self::from_reader(file)
    }
    /// Convenience method for parsing credentials from json str
    pub fn from_str(s: &str) -> Result<Credentials> {
        serde_json::from_str(s).map_err(Error::from)
    }
    /// Convenience method for parsing credentials from a reader
    /// ( i.e. a `std::fs:File` )
    pub fn from_reader<R>(r: R) -> Result<Credentials>
    where
        R: Read,
    {
        serde_json::from_reader(r).map_err(Error::from)
    }
}

#[derive(Default, Serialize, Deserialize, PartialEq, Debug)]
struct Claims {
    iss: String,
    scope: String,
    aud: String,
    exp: i64,
    iat: i64,
}

/// An access token can be used to authenticate
/// google api requests
///
/// Instances of these can be onbtained from one of the methods provided by
/// `gauthz.Tokens`
#[derive(Default, Deserialize, PartialEq, Debug, Clone)]
pub struct AccessToken {
    access_token: String,
    expires_in: u64,
    #[serde(default, skip)]
    instant: Option<Instant>,
    #[serde(default, skip)]
    duration: Option<Duration>,
}

impl AccessToken {
    /// Returns string value of access token
    ///
    /// This is typically the value you use for HTTP Authorization: Bearer
    /// header values
    pub fn value(&self) -> &str {
        &self.access_token
    }
    /// Returns true if this access token has has expired
    ///
    /// This is typically one hour in practice
    pub fn expired(&self) -> bool {
        match (self.instant, self.duration) {
            (Some(inst), Some(dur)) => inst.elapsed() >= dur,
            _ => false,
        }
    }

    fn start(mut self) -> Self {
        self.instant = Some(Instant::now());
        self.duration = Some(Duration::from_secs(self.expires_in));
        self
    }
}

/// An interface for generating access tokens to authenticate
/// google api requests
///
/// A scope is required to limit access to target apis
/// some scopes, like https://www.googleapis.com/auth/cloud-platform,
/// provide access to multiple apis
#[derive(Clone)]
pub struct Tokens<C>
where
    C: Connect + Clone,
{
    http: HyperClient<C>,
    credentials: Credentials,
    scopes: String,
}

#[cfg(feature = "tls")]
impl Tokens<HttpsConnector<HttpConnector>> {
    /// Creates a new instance of `Tokens` using a `hyper::Client`
    /// preconfigured for tls.
    ///
    /// For client customization use `Tokens::custom` instead
    pub fn new<Iter>(
        handle: &Handle,
        credentials: Credentials,
        scopes: Iter,
    ) -> Self
    where
        Iter: ::std::iter::IntoIterator<Item = Scope>,
    {
        let connector = HttpsConnector::new(4, handle).unwrap();
        let hyper = HyperClient::configure()
            .connector(connector)
            .keep_alive(true)
            .build(handle);
        Tokens::custom(hyper, credentials, scopes)
    }
}

impl<C: Connect + Clone> Tokens<C> {
    /// Creates a new instance of `Tokens` with a custom `hyper::Client`
    /// with a customly configured `hyper::Client`
    pub fn custom<Iter>(
        http: HyperClient<C>,
        credentials: Credentials,
        scopes: Iter,
    ) -> Self
    where
        Iter: ::std::iter::IntoIterator<Item = Scope>,
    {
        Self {
            http,
            credentials,
            scopes: scopes
                .into_iter()
                .map(|s| s.url())
                .collect::<Vec<_>>()
                .join(","),
        }
    }

    /// Returns a `Stream` of `AccessTokens`. The same `AccessToken` will be
    /// yielded multiple times until it is expired. After which, a new token
    /// will be fetched
    pub fn stream(&self) -> Stream<AccessToken> {
        let instance = self.clone();
        let tokens =
            stream::unfold::<
                _,
                _,
                Future<(AccessToken, Option<AccessToken>)>,
                _,
            >(None, move |state| {
                Some(match state {
                    Some(ref token) if !token.expired() => {
                        Box::new(future::ok((token.clone(), state.clone())))
                    }
                    _ => {
                        Box::new(instance.get().map(|token| {
                            let next = Some(token.clone());
                            (token, next)
                        }))
                    }
                })
            });
        Box::new(tokens)
    }

    fn new_request(&self) -> Request {
        let header: Header<()> = Header {
            alg: Algorithm::RS256,
            ..Default::default()
        };
        let iat = time::now_utc().to_timespec().sec;
        let exp = iat + 3600;
        let payload = Payload {
            claims: Some(Claims {
                iss: self.credentials.clone().client_email,
                scope: self.scopes.clone(),
                aud: TOKEN_URL.into(),
                exp: exp,
                iat: iat,
            }),
            ..Default::default()
        };
        let signed = Token::new(header, payload)
            .sign(&self.credentials.clone().private_key.into_bytes())
            .unwrap();
        let mut req = Request::new(Method::Post, TOKEN_URL.parse().unwrap());
        req.headers_mut().set(ContentType::form_url_encoded());
        let body = format!(
            "grant_type=urn%3Aietf%3Aparams%3Aoauth%3Agrant-type%3Ajwt-bearer&assertion={assertion}",
            assertion = signed.as_str()
        );
        req.set_body(body);
        req
    }

    /// Returns a `Future` yielding a new `AccessToken`
    pub fn get(&self) -> Future<AccessToken> {
        Box::new(
            self.http
                .request(self.new_request())
                .map_err(Error::from)
                .and_then(|response| {
                    let status = response.status();
                    let body = response.body().concat2().map_err(Error::from);
                    body.and_then(move |body| if status.is_success() {
                        serde_json::from_slice::<AccessToken>(&body)
                            .map_err(|err| ErrorKind::Codec(err).into())
                            .map(AccessToken::start)
                    } else {
                        Err(match serde_json::from_slice::<ApiError>(&body) {
                            Err(err) => ErrorKind::Codec(err).into(),
                            Ok(err) => {
                                ErrorKind::Api(err.error, err.error_description)
                                    .into()
                            }
                        })
                    })
                }),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::AccessToken;
    use std::time::Duration;


    #[test]
    fn tokens_value() {
        let token = AccessToken {
            access_token: "test".into(),
            expires_in: 1,
            ..Default::default()
        };
        assert_eq!(token.value(), token.access_token)
    }

    #[test]
    fn tokens_expire() {
        let token = AccessToken {
            access_token: "test".into(),
            expires_in: 1,
            ..Default::default()
        }.start();
        assert!(!token.expired());
        let duration = Duration::from_secs(1);
        assert_eq!(token.duration, Some(duration));
        ::std::thread::sleep(duration);
        assert!(token.expired())
    }
}
