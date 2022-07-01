
use std::{thread, time::Duration};

use serde::{Serialize, Deserialize};
use webserver::{http_server::HTTPServer, route::Route, request::Request, response::Response};


fn main() {
    let mut server = HTTPServer::new("127.0.0.1", 8080, 4);

    // Set up all endpoints
    server.routes(vec![
        Route::get("/", index_handler),
        Route::get("/slow", slow_handler),
    ]);

    server.listen(|this| {
        println!("Now listening on http://{}:{}", 
            this.get_host(), 
            this.get_port()
        );
    });

    loop {
        
    }
}

#[derive(Serialize, Deserialize,)]
struct IndexDTO {
    name: String,
    age: u8,
    phones: Vec<String>
}

fn index_handler(req: &Request, res: &mut Response) {
    let data = IndexDTO {
        name: "John Doe".to_owned(),
        age: 43,
        phones: [
            "+44 1234567".to_owned(),
            "+44 2345678".to_owned()
        ].to_vec()
    };

    res.set_header("Some-Custom-Header", "Some-Value");

    res.json(data);
}

fn slow_handler(req: &Request, res: &mut Response) {
    println!("Index!\n{:?}", req.get_headers());
    
    thread::sleep(Duration::from_secs(5));

    let data = IndexDTO {
        name: "John Doe".to_owned(),
        age: 43,
        phones: [
            "+44 1234567".to_owned(),
            "+44 2345678".to_owned()
        ].to_vec()
    };

    res.json(data);
}