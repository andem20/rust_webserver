use webserver::{http_server::HTTPServer, request::Request, response::Response, route::Route};

#[path ="./test_data.rs"]
mod test_data;

pub fn test_handler(req: &Request, res: &mut Response) {
    let data = test_data::get_data();

    res.json(data);
}

pub fn setup() -> HTTPServer {
    let mut server = HTTPServer::new("127.0.0.1", 8080, 4);

    // Set up all endpoints
    server.routes(vec![
        Route::get("/", test_handler),
    ]);

    server.listen(|this| {
        println!("Now listening on http://{}:{}", 
            this.get_host(), 
            this.get_port()
        );
    });

    server
}