use std::{net::{TcpListener, TcpStream}, io::{Read, Write}, fs, time::Duration};

use crate::http_server::HTTPServer;

mod http_server;

const HOST: &str = "127.0.0.1";
const PORT: &str = "8000";

fn main() {

    let mut server = HTTPServer::new("127.0.0.1", 8000);

    // Set up all endpoints

    server.listen(|this| {
        println!("Now listening on http://{}:{}", 
            this.get_host(), 
            this.get_port()
        );
    });

    // let listener = TcpListener::bind(format!("{}:{}", &HOST, &PORT)).unwrap();

    // println!("Now listening on http://{}:{}\n\n", &HOST, &PORT);

    // for stream in listener.incoming() {
    //     let stream = stream.unwrap();

    //     on_connection(stream);
    // }
}

fn on_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];

    stream.read(&mut buffer).unwrap();

    let headers = String::from_utf8_lossy(&buffer);

    println!("Request: {}", &headers);

    get("/", "index.html", &headers, &mut stream);

}


fn get(endpoint: &str, filename: &str, headers: &str, stream: &mut TcpStream) {
    let views_dir = "public/views/";
    
    let (status, file) = match headers.split(" ").nth(1) {
        Some("/") => ("200 OK", "index.html"),
        Some("/slow") => {
            std::thread::sleep(Duration::from_millis(5000));
            ("200 OK", "slow.html")
        },
        _ => ("404 NOT FOUND", "404.html")
    };

    let contents_path = format!("{}/{}", &views_dir, file);
    let contents = fs::read_to_string(contents_path).unwrap();

    let response = format!(
        "HTTP/1.1 {}\r\nContent-Length: {}\r\n\r\n{}",
        status,
        contents.len(),
        contents
    );

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}