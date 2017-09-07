# gauthz [![Build Status](https://travis-ci.org/softprops/gauthz.svg?branch=master)](https://travis-ci.org/softprops/gauthz) [![Coverage Status](https://coveralls.io/repos/github/softprops/gauthz/badge.svg)](https://coveralls.io/github/softprops/gauthz)

> google authentication by way of rust

## [Documentation](https://softprops.github.io/gauthz)

## Install

...

## Usage

Typical use requires `gauthz::Tokens` instance with a given `hyper::Client`,
`gauthz::Credentials` and `gauthz::Scope`s representing the access level for
your intended API usage.

> Note your `hyper::Client` _must_ be configured with tls support. Hyper doesn't
provide that out of the box but there are multiple crates that provide the support

A `gauthz::Tokens` instance provides two interfaces `get` which returns a `Future`
that reolves to an access token and a `Stream` which resolves to new tokens when
the current token expires ( typically ) after one hour. The stream interface is
intended for long running applications which will inevitably require access for more
than one hour.

```rust
extern crate hyper;
extern crate futures;
extern crate gauthz;
extern crate tokio_core;
extern crate hyper_tls;

use futures::Future;
use hyper_tls::HttpsConnector;
use tokio_core::reactor::Core;

use gauthz::{Credentials, Result, Scope, Tokens};
use gauthz::error::ErrorKind;

fn run() -> Result<String> {
    let mut core = Core::new()?;
    let tokens = Tokens::new(
        hyper::Client::configure()
            .connector(HttpsConnector::new(4, &core.handle()).map_err(|_| {
                ErrorKind::Msg("failed to create https connector".into())
            })?)
            .build(&core.handle()),
        Credentials::default().unwrap(),
        vec![Scope::CloudPlatform],
    );
    core.run(tokens.get().map(|t| t.value().to_owned()))
}

fn main() {
    match run() {
        Ok(ok) => println!("{}", ok),
        Err(err) => println!("{}", err),
    }
}
```

## pronounced

> gawthz

Doug Tangren (softprops) 2017