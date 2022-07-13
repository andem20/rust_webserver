#[derive(Clone, PartialEq, Hash, Eq, Debug)]
pub enum Method {
    GET,
    POST,
}


impl Method {
    pub fn from_string(string: &str) -> Method {
        match string {
            "GET" => Method::GET,
            "POST" => Method::POST,
            _ => Method::GET
        }
    }
}