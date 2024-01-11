use std::{
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};

pub fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    println!("Running on port 7878");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                // let stream = stream.unwrap();
                println!("Connection established");
                println!("Peer Addr: {:#?}", stream.peer_addr());

                handle_connection(stream).expect("Unhandled");
            }
            Err(_e) => { /* connection failed */ }
        }
    }
    Ok(())
}

fn handle_connection(mut stream: TcpStream) -> std::io::Result<()> {
    let mut buffer = [0; 100];
    stream.read(&mut buffer).unwrap();
    println!("Received from client: {}", String::from_utf8_lossy(&buffer));
    stream.write(&mut buffer).unwrap();
    Ok(())
}

#[allow(dead_code)]
fn http_handle_connection(mut stream: TcpStream) -> std::io::Result<()> {
    let buf_reader = BufReader::new(&mut stream);
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();
    println!("Request: {:#?}", http_request);
    let s: &[u8] = &[1];
    let res = stream.write(s)?;
    println!("Response: {:#?}", res);
    Ok(())
}
