use std::{net::{TcpListener, TcpStream}, io::Read};

fn main() {
    let host = "127.0.0.1";
    let port = "8000";
    let addr = format!("{}:{}", &host, &port);

    let listener = TcpListener::bind(&addr).unwrap();

    println!("Now listening on {}:{}", &host, &port);

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        on_connection(stream);
    }
}

fn on_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];

    println!("Connection established!");

    stream.read(&mut buffer).unwrap();

    let headers = String::from_utf8_lossy(&buffer);

    println!("Request: {}", headers);
}