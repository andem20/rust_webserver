use std::{sync::{Mutex, Arc, mpsc::{Receiver, self}}, net::TcpListener, thread, io};

use crate::{threadpool::ThreadPool, tcp_server::route::Route};

use super::connection_handler::connection_handler;

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
                            connection_handler(s, root_route); 
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