use httparse::Request;
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use std::thread::sleep;
use std::time::Duration;

fn main() {
    let server = TcpListener::bind("127.0.0.1:8080").unwrap();
    for stream in server.incoming() {
        let stream = stream.unwrap();
        handler(stream);
    }
}

fn handler(mut stream: TcpStream) {
    let mut buffer = [0; 2048];

    let read_size = stream.read(&mut buffer).unwrap();
    let response = "HTTP/1.1 200 OK\r\n\r\n";
    let mut headers = [httparse::EMPTY_HEADER; 512];
    let request = {
        let mut req = Request::new(&mut headers);
        req.parse(&buffer);
        req
    };

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
