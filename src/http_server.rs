use std::{net::{TcpListener, TcpStream}, io::{Read, Write, self}, collections::HashMap, thread, time::Duration, sync::{mpsc::{Receiver, self, Sender}, Arc, Mutex}};

use serde_json::json;

use crate::{route::Route, request::Request, response::{Response, self}, threadpool::ThreadPool};

pub type Headers = HashMap<String, String>;


pub struct HTTPServer {
    host: String,
    port: u16,
    routes: HashMap<String, Route>,
    terminate: Arc<Mutex<bool>>,
    receiver: Option<Receiver<bool>>,
    pool_size: u16,
}

impl HTTPServer {
    pub fn new(host: &str, port: u16, pool_size: u16) -> HTTPServer {
        HTTPServer {
            host: host.to_string(),
            port,
            routes: HashMap::new(),
            terminate: Arc::new(Mutex::new(false)),
            receiver: None,
            pool_size
        }
    }
    
    pub fn listen(&mut self, callback: fn(this: &HTTPServer)) {
        let addr = format!("{}:{}", self.host, self.port);
        callback(&self);

        let routes = Arc::new(self.routes.clone());

        let pool = ThreadPool::new(self.pool_size); // should be 2x num of cpu cores

        let listener = TcpListener::bind(addr).unwrap();

        let terminate = self.terminate.clone();

        let (sender, receiver) = mpsc::channel();

        self.receiver = Some(receiver);

        listener.set_nonblocking(true).unwrap();
        
        thread::spawn(move || {
            loop {
                let routes = routes.clone();
                let stream = listener.accept();

                match stream {
                    Ok((s, _addr)) => {
                        pool.execute(|| {
                            handle_connection(s, routes); 
                        });
        
                        let active_threads = pool.get_num_active_threads();
        
                        if active_threads == 0 && *terminate.lock().unwrap() {
                            let _ = sender.send(true);
                        }

                    }
                    Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                        if *terminate.lock().unwrap() { break }
                        continue;
                    }
                    Err(e) => panic!("encountered IO error: {e}"),
                }
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
        let _ = self.receiver.as_ref().unwrap().recv();
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
        let value = response.get_content();

        (200, value)
    };

    let mut headers = String::new();
    for header in response.get_headers() {
        headers.push_str(&header.0);
        headers.push_str(": ");
        headers.push_str(&header.1);
        headers.push_str("\r\n");
    }

    let response = format!(
        "HTTP/1.1 {}\r\nContent-Length: {}\r\n{}\r\n{}",
        status,
        &content.len(),
        &headers,
        &content
    );

    println!("{}", &response);

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}