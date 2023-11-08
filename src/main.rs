#![allow(dead_code)]
mod macros;
mod events;
mod server;
mod client;

fn main() {
    server::serve::start_server();
    // events::keyboard::main();
}