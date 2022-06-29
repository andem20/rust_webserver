use std::{net::{TcpListener, TcpStream}, io::{Read, Write}, collections::HashMap, thread, time::Duration, sync::{mpsc::{Receiver, self, Sender}, Arc, Mutex}};

use serde_json::json;

use crate::{route::Route, request::Request, response::Response, threadpool::ThreadPool};

pub type Headers = HashMap<String, String>;


pub struct HTTPServer {
    host: String,
    port: u16,
    routes: HashMap<String, Route>,
    terminate: Arc<Mutex<bool>>
}

impl HTTPServer {
    pub fn new(host: &str, port: u16) -> HTTPServer {
        HTTPServer {
            host: host.to_string(),
            port,
            routes: HashMap::new(),
            terminate: Arc::new(Mutex::new(false))
        }
    }
    
    pub fn listen(&self, callback: fn(this: &HTTPServer)) {
        let addr = format!("{}:{}", self.host, self.port);
        callback(&self);

        let routes = Arc::new(self.routes.clone());

        let pool = ThreadPool::new(4); // should be 2x num of cpu cores

        let listener = TcpListener::bind(addr).unwrap();

        let terminate = self.terminate.clone();
        
        thread::spawn(move || {
            for stream in listener.incoming() {

                if *terminate.lock().unwrap() { break }

                let stream = stream.unwrap();
                let routes = routes.clone();
                
                pool.execute(|| {
                    handle_connection(stream, routes); 
                });
            }
        });
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

    pub fn close(&mut self) {
        *self.terminate.lock().unwrap() = true;
    }
    
}

fn handle_connection(mut stream: TcpStream, routes: Arc<HashMap<String, Route>>) {
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