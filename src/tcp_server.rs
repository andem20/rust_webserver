use std::{net::{TcpListener, TcpStream}, io::{Read, Write, self}, collections::HashMap, thread, sync::{mpsc::{Receiver, self}, Arc, Mutex}, borrow::{BorrowMut, Borrow}, time::Duration, cell::RefCell, fmt::format};

use serde_json::json;

use crate::{route::{Route, self}, request::Request, response::{Response}, threadpool::ThreadPool};

pub type Headers = HashMap<String, String>;


pub struct TCPServer {
    host: String,
    port: u16,
    root_route: Route,
    terminate: Arc<Mutex<bool>>,
    receiver: Option<Receiver<bool>>,
    pool_size: u16,
}

impl TCPServer {
    pub fn new(host: &str, port: u16, pool_size: u16) -> TCPServer {
        TCPServer {
            host: host.to_string(),
            port,
            root_route: Route::new("", None, None),
            terminate: Arc::new(Mutex::new(false)),
            receiver: None,
            pool_size
        }
    }
    
    pub fn listen(&mut self, callback: fn(this: &TCPServer)) {
        let addr = format!("{}:{}", self.host, self.port);
        callback(&self);

        let root_route = Arc::new(self.root_route.clone());

        let pool = ThreadPool::new(self.pool_size);

        let listener = TcpListener::bind(addr).unwrap();

        let terminate = self.terminate.clone();

        let (sender, receiver) = mpsc::channel();

        self.receiver = Some(receiver);

        listener.set_nonblocking(true).unwrap();
        
        thread::spawn(move || {
            loop {
                let root_route = root_route.clone();
                let stream = listener.accept();

                match stream {
                    Ok((s, _addr)) => {
                        pool.execute(|| {
                            handle_connection(s, root_route); 
                        });
        
                        let active_threads = pool.get_num_active_threads();
        
                        if active_threads == 0 && *terminate.lock().unwrap() {
                            let _ = sender.send(true);
                        }

                    }
                    Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                        if *terminate.lock().unwrap() { break }
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
            let path = route.get_endpoint().split("/");
            let mut current_route = &mut self.root_route;
            
            for branch in path {
                let mut key = format!("/{}", branch);
                let endpoint = key.clone();

                if branch.contains(":") {
                    key = "/:".to_owned();
                }
                
                if !current_route.get_routes().contains_key(&key) {
                    current_route.add_route(&key, Route::new(&endpoint, None, None));
                }

                current_route = current_route.get_route(&key).unwrap();
            }

            current_route.set_handler(route.get_handler().unwrap());
            current_route.set_method(route.get_method().as_ref().unwrap().to_owned());
        }
    }

    pub fn close(&mut self) {
        *self.terminate.lock().unwrap() = true;
        let _ = self.receiver.as_ref().unwrap().recv();
    }
    
}

fn handle_connection(mut stream: TcpStream, routes: Arc<Route>) {
    let mut buffer = [0; 1024];

    stream.read(&mut buffer).unwrap();

    let headers = String::from_utf8_lossy(&buffer);

    let mut request = Request::new(&buffer);
    let mut response = Response::new(HashMap::new());
    
    let endpoint = headers.split(" ").nth(1).unwrap();

    let url = endpoint.split("/");

    let mut route = routes.clone();
    let mut valid = true;

    for branch in url {
        let key = format!("/{}", branch);
        if route.get_routes().contains_key(&key) {
            route = Arc::new(route.get_routes().get(&key).unwrap().clone());
            continue;
        }

        if route.get_routes().contains_key("/:") {
            // TODO maybe should check type as well?

            // Remove leading slash
            let param = &route.get_routes().get("/:").unwrap().get_endpoint()[2..];
            println!("param: {}", param);
            request.set_param(param, branch);
            route = Arc::new(route.get_routes().get("/:").unwrap().clone());
            continue;
        }
        
        valid = false;
        break;
    }

    // let route = routes.get_endpoint();
    
    let (status, content) = if !valid {
        let value = serde_json::to_string_pretty(&json!({
            "error": "Page not found"
        })).unwrap();

        (404, value)
    } else {
        let handler = route.get_handler().unwrap();
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