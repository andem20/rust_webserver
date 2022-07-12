
use webserver::tcp_server::{request::Request, response::Response};

use crate::util::test_data;

pub fn get_handler(req: &Request, res: &mut Response) {
    let data = test_data::get_data();

    res.json(data);
}

pub fn users_handler(req: &Request, res: &mut Response) {
    let id = req.get_param("id");

    if id.is_some() {
        res.json(id.unwrap());
    }
}