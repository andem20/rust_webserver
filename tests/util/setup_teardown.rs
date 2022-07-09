use webserver::{tcp_server::TCPServer, route::Route};

pub fn setup(routes: Vec<Route>, port: u16) -> TCPServer {
    let mut server = TCPServer::new("127.0.0.1", port, 4);

    // Set up all endpoints
    server.routes(routes);

    server.listen(|this| {
        println!("Now listening on http://{}:{}", 
            this.get_host(), 
            this.get_port()
        );
    });

    server
}