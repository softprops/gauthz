extern crate futures;
extern crate gauthz;
extern crate tokio_core;

use futures::Future;
use tokio_core::reactor::Core;

use gauthz::{Credentials, Result, Scope, Tokens};

fn run() -> Result<String> {
    let mut core = Core::new()?;
    let tokens = Tokens::new(
        &core.handle(),
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
