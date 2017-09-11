# gauthz [![Build Status](https://travis-ci.org/softprops/gauthz.svg?branch=master)](https://travis-ci.org/softprops/gauthz) [![Coverage Status](https://coveralls.io/repos/github/softprops/gauthz/badge.svg)](https://coveralls.io/github/softprops/gauthz)

> google api service authentication by way of rust

## [Documentation](https://softprops.github.io/gauthz)

## Install

Add the following to your project's `Cargo.toml` file

```toml
[dependencies]
gauthz = "0.1"
```

## Usage

Typical use requires `gauthz::Tokens` configured with a tokio reactor handle
`gauthz::Credentials` and `gauthz::Scope`s representing the access level for
your intended API usage.

> Note your `hyper::Client` _must_ be configured with tls support. Hyper doesn't
provide that out of the box but there are multiple crates that provide the support

A `gauthz::Tokens` instance provides two interfaces `get` which returns a `Future`
that resolves to an access token and a `Stream` which resolves to new tokens when
the current token expires ( typically ) after one hour. The stream interface is
intended for long running applications which will inevitably require access for more
than one hour.

```rust
extern crate futures;
extern crate gauthz;
extern crate tokio_core;

use futures::Future;
use tokio_core::reactor::Core;

use gauthz::{Credentials, Result, Scope, Tokens};
use gauthz::error::ErrorKind;

fn run() -> Result<String> {
    let mut core = Core::new()?;
    let tokens = Tokens::new(
        &core.handle(),
        Credentials::default().unwrap(),
        vec![Scope::CloudPlatform],
    );
    core.run(
        tokens.get().map(
            |t| t.value().to_owned()
        )
    )
}

fn main() {
    match run() {
        Ok(ok) => println!("{}", ok),
        Err(err) => println!("{}", err),
    }
}
```

These tokens may be used in `Authorization` HTTP headers to authenticate with
the Google API's the tokens are scoped to.

## pronounced

> gawthz

Doug Tangren (softprops) 2017