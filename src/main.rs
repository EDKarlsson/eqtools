#![allow(dead_code)]
// use rdev::{EventType,Key};
mod macros;
mod events;
mod server;
mod client;
use std::error::Error;

#[async_std::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let _ = server::p2p::main();
    Ok(())
}
