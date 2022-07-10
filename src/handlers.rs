use std::{thread, time::Duration};

use serde::{Serialize, Deserialize};

use crate::tcp_server::{request::Request, response::Response};

#[derive(Serialize, Deserialize,)]
struct IndexDTO {
    name: String,
    age: u8,
    phones: Vec<String>
}

pub fn index_handler(req: &Request, res: &mut Response) {
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

pub fn slow_handler(req: &Request, res: &mut Response) {
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