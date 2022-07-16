
use webserver::tcp_server::{request::Request, response::Response};

use crate::util::test_data::{self, TestDTO};

pub fn get_handler(req: &Request, res: &mut Response) {
    let data = test_data::get_data();

    res.json(data);
}

pub fn post_handler(req: &Request, res: &mut Response) {
    let data = req.get_body().as_ref().unwrap();
    let data: TestDTO = serde_json::from_str(data).unwrap();

    res.json(data);
}

pub fn users_handler(req: &Request, res: &mut Response) {
    let id = req.get_param("id");

    if id.is_some() {
        res.json(id.unwrap());
    }
}