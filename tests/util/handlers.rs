use std::collections::HashMap;

use webserver::{request::Request, response::Response};

use crate::util::test_data;

pub fn index_handler(req: &Request, res: &mut Response) {
    let data = test_data::get_data();

    res.json(data);
}

pub fn users_handler(req: &Request, res: &mut Response) {
    let id = req.get_param("id");

    if id.is_some() {
        res.json(id.unwrap());
    }
}