use webserver::{http_server::HTTPServer, route::Route};

pub fn setup(routes: Vec<Route>, port: u16) -> HTTPServer {
    let mut server = HTTPServer::new("127.0.0.1", port, 4);

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