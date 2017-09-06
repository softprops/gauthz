extern crate hyper;
extern crate futures;
extern crate gauthz;
extern crate tokio_core;
extern crate hyper_tls;

use futures::Future;
use hyper_tls::HttpsConnector;
use std::env;
use std::fs::File;
use tokio_core::reactor::Core;

use gauthz::{Credentials, Result, Tokens};
use gauthz::error::ErrorKind;

fn run() -> Result<String> {
    let mut core = Core::new()?;
    let file =
        File::open(env::var("GOOGLE_APPLICATION_CREDENTIALS").map_err(|_| {
            ErrorKind::Msg("missing GOOGLE_APPLICATION_CREDENTIALS".into())
        })?)?;

    let tokens = Tokens::new(
        hyper::Client::configure()
            .connector(HttpsConnector::new(4, &core.handle()).map_err(|_| {
                ErrorKind::Msg("failed to create https connector".into())
            })?)
            .build(&core.handle()),
        Credentials::from_reader(file).unwrap(),
        "https://www.googleapis.com/auth/cloud-platform",
    );
    core.run(tokens.get().map(|t| t.value().to_owned()))
}

fn main() {
    match run() {
        Ok(ok) => println!("{}", ok),
        Err(err) => println!("{}", err),
    }
}
