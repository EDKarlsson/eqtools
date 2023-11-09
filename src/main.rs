#![allow(dead_code)]
use rdev::{EventType,Key};
mod macros;
mod events;
mod server;
mod client;

fn main() {
    // server::serve::start_server();
    events::keyboard::send(&EventType::KeyPress(Key::KeyS));
    events::keyboard::send(&EventType::KeyRelease(Key::KeyS));
    events::keyboard::send(&EventType::KeyPress(Key::Space));
    events::keyboard::send(&EventType::KeyRelease(Key::Space));

    events::keyboard::send(&EventType::KeyPress(Key::KeyS));
    events::keyboard::send(&EventType::KeyRelease(Key::KeyS));
    events::keyboard::send(&EventType::KeyPress(Key::Space));
    events::keyboard::send(&EventType::KeyRelease(Key::Space));

    events::keyboard::send(&EventType::KeyPress(Key::KeyS));
    events::keyboard::send(&EventType::KeyRelease(Key::KeyS));
    events::keyboard::send(&EventType::KeyPress(Key::Space));
    events::keyboard::send(&EventType::KeyRelease(Key::Space));

    // events::keyboard::main();
}
