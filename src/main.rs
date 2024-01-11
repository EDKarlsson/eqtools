#![allow(dead_code)]
// use rdev::{EventType,Key};
mod macros;
mod events;
mod server;
mod client;
use std::error::Error;

#[async_std::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // events::keyboard::send(&EventType::KeyPress(Key::KeyS));
    // events::keyboard::send(&EventType::KeyRelease(Key::KeyS));
    // events::keyboard::send(&EventType::KeyPress(Key::Space));
    // events::keyboard::send(&EventType::KeyRelease(Key::Space));
    // events::keyboard::send(&EventType::KeyPress(Key::Return));

    // server::serve::start_server();
    // events::keyboard::main();
    let _ = server::p2p::main();
    Ok(())
}
