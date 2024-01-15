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

pub fn start_server() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        println!("Peer Addr: {:#?}", stream.peer_addr());
        handle_connection(stream).expect("Unhandled");
        println!("Hello...");
    }
}

fn handle_connection(mut stream: TcpStream) -> std::io::Result<()>{
    let buf_reader = BufReader::new(&mut stream);
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();
    println!("Request: {:#?}", http_request);
    let _s: &[u8] = &[1];
    // let _res = stream.write(s)?;
    let res = stream.write(&[1])?;
    println!("Response: {:#?}", res);
    Ok(())
}

fn handle_request(http_request: Vec<String>) {
    println!("Request: {:#?}", http_request);
}
