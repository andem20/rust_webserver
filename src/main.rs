use std::{time::Duration, thread};

use response::Response;
use route::Route;
use serde::{Serialize, Deserialize};
use serde_json::{json, Value, Error};

use crate::http_server::HTTPServer;

mod http_server;
mod route;
mod response;

fn main() {
    let mut server = HTTPServer::new("127.0.0.1", 8080);

    // Set up all endpoints
    server.routes(vec![
        Route::get::<IndexDTO>("/", index_handler),
        // Route::get("/test", test_handler),
        // Route::get("/test/slow", slow_handler),
    ]);

    server.listen(|this| {
        println!("Now listening on http://{}:{}", 
            this.get_host(), 
            this.get_port()
        );
    });
}



#[derive(Serialize, Deserialize,)]
struct IndexDTO {
    name: String,
    age: u8,
    phones: Vec<String>
}


impl Response for IndexDTO {
    fn get_value(&self) -> Result<String, Error> {
        serde_json::to_string_pretty(self)
    }
}


fn index_handler() -> Box<dyn Response> {
    println!("Index!");

    Box::new(IndexDTO {
        name: "John Doe".to_owned(),
        age: 43,
        phones: [
            "+44 1234567".to_owned(),
            "+44 2345678".to_owned()
        ].to_vec()
    })
}

fn test_handler() -> Value {
    println!("Test!");
    json!({
        "test": "TEST"
    })
}

fn slow_handler() -> Value {
    thread::sleep(Duration::from_secs(5));
    println!("Slow!");

    json!({
        "sow": "SLOW"
    })
}