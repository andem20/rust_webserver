use std::{net::{TcpListener, TcpStream}, io::{Read, Write}, collections::HashMap, sync::{Arc, Mutex, MutexGuard}, thread, time::Duration};

use serde_json::json;

use crate::{route::Route, request::Request, response::Response};

pub type Headers = HashMap<String, String>;


pub struct HTTPServer {
    host: String,
    port: u16,
    listener: Option<TcpListener>,
    routes: HashMap<String, Route>,
}

impl HTTPServer {
    pub fn new(host: &str, port: u16) -> HTTPServer {
        HTTPServer {
            host: host.to_string(),
            port,
            listener: None,
            routes: HashMap::new(),
        }
    }
    
    pub fn listen(&mut self, callback: fn(this: &HTTPServer)) {
        let listener = TcpListener::bind(format!("{}:{}", self.host, self.port)).unwrap();
        self.listener = Some(listener);
        
        let listener = self.listener.as_ref().unwrap();

        callback(self);
        
        for stream in listener.incoming() {
            let routes = self.routes.clone();
            
            match stream {
                Ok(s) => {
                    thread::spawn(|| {
                        handle_connection(s, routes); 
                    });
                }
                Err(e) => panic!("encountered IO error: {}", e),
            }
        }
    }

    pub fn get_host(&self) -> &str {
        self.host.as_ref()
    }

    pub fn get_port(&self) -> u16 {
        self.port
    }

    pub fn routes(&mut self, routes: Vec<Route>) {
        for route in routes {
            self.routes.insert(route.get_endpoint().clone(), route);
        }
    }
    
}

fn handle_connection(mut stream: TcpStream, routes: HashMap<String, Route>) {
    let mut buffer = [0; 1024];

    stream.read(&mut buffer).unwrap();

    let headers = String::from_utf8_lossy(&buffer);

    let request = Request::new(&buffer);
    let mut response = Response::new(HashMap::new());
    
    let endpoint = headers.split(" ").nth(1).unwrap();

    let route = routes.get(endpoint);
    
    let (status, content) = if route.is_none() {
        let value = serde_json::to_string_pretty(&json!({
            "error": "Page not found"
        })).unwrap();

        (404, value)
    } else {
        let handler = route.unwrap().get_handler();
        handler(&request, response.as_mut());

        (200, response.get_content())
    };

    let response = format!(
        "HTTP/1.1 {}\r\nContent-Length: {}\r\n\r\n{}\r\nContent-Type: application/json",
        status,
        &content.len(),
        &content
    );

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}