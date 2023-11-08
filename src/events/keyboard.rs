use rdev::{listen, Event};

pub fn main() {
// This will block.
    if let Err(error) = listen(callback) {
        println!("Error: {:?}", error)
    }
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