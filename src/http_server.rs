use std::{collections::HashMap, net::{TcpListener, TcpStream}, io::Read};

pub struct HTTPServer {
    host: String,
    port: u16,
    listener: Option<TcpListener>,
    endpoints: HashMap<String, fn()>,
}

impl HTTPServer {
    pub fn new(host: &str, port: u16) -> HTTPServer {
        HTTPServer {
            host: host.to_string(),
            port,
            listener: None,
            endpoints: HashMap::new()
        }
    }
    
    pub fn listen(&mut self, callback: fn(this: &HTTPServer)) {
        let listener = TcpListener::bind(format!("{}:{}", self.host, self.port)).unwrap();
        self.listener = Some(listener);
        
        let listener = self.listener.as_ref().unwrap();

        callback(self);

        for stream in listener.incoming() {
            let stream = stream.unwrap();
    
            on_connection(stream);
            // on_request();
        }
    }

    pub fn get_listener(&self) -> Option<&TcpListener> {
        self.listener.as_ref()
    }

    pub fn get_host(&self) -> &str {
        self.host.as_ref()
    }

    pub fn get_port(&self) -> u16 {
        self.port
    }
}

fn on_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];

    stream.read(&mut buffer).unwrap();

    let headers = String::from_utf8_lossy(&buffer);

    println!("Request: {}", &headers);
}