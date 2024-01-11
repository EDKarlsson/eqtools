use std::io::{Read, Write};
use std::net::TcpStream;
use std::str;
fn main() {
    let mut stream = TcpStream::connect("127.0.0.1:7878").unwrap();
    let msg_to_send = "Hello from TCP client";

    stream.write(msg_to_send.as_bytes()).unwrap();

    let mut buffer = [0; 200];
    stream.read(&mut buffer).unwrap();

    println!(
        "Got echo back from server:{:?}",
        str::from_utf8(&buffer)
            .unwrap()
            .trim_end_matches(char::from(0))
    );
}