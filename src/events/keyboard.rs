#![allow(unused)]
use rdev::{listen, Event, simulate, Button, EventType, Key, SimulateError};
use std::{thread, time};

pub fn main() {
    // events::keyboard::send(&EventType::KeyPress(Key::KeyS));
    // events::keyboard::send(&EventType::KeyRelease(Key::KeyS));
    // events::keyboard::send(&EventType::KeyPress(Key::Space));
    // events::keyboard::send(&EventType::KeyRelease(Key::Space));
    // events::keyboard::send(&EventType::KeyPress(Key::Return));

    // events::keyboard::main();

    // This will block.
    if let Err(error) = listen(callback) {
        println!("Error: {:?}", error)
    }
}

pub fn send(event_type: &EventType) {
    let delay = time::Duration::from_millis(20);
    match simulate(event_type) {
        Ok(()) => (),
        Err(SimulateError) => {
            println!("We could not send {:?}", event_type);
        }
    }
    // Let ths OS catchup (at least MacOS)
    thread::sleep(delay);
}


fn handle_request(req: String) {
    println!("User wrote {:?}", req);
}

fn callback(event: Event) {
    println!("My callback {:?}", event);
    match event.name {
        Some(string) => handle_request(string),
        None => (),
  }
}
