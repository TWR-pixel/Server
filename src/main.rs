use std::io::prelude::*;
use std::net::TcpStream;
use std::net::TcpListener;

fn main() {
    let listener = TcpListener::bind("5.63.157.232:3333").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        
        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 512];

    stream.read(&mut buffer).unwrap();

    let str = String::from_utf8_lossy(&buffer[..]);

    if str.chars().nth(0).unwrap() == '9' {
        println!("Request: {}", String::from_utf8_lossy(&buffer[..]));
    }
    else {
        println!("err");
    }
}