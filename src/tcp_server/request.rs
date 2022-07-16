use std::{collections::HashMap};

use crate::tcp_server::method::Method;

use super::headers_type::Headers;

pub struct Request {
    headers: Headers,
    params: HashMap<String, String>,
    method: Method,
    body: Option<String>
}

impl Request {
    pub fn new(buffer: &[u8]) -> Request {
        let mut headers_map = HashMap::new();
        let mut body = None;
        
        let headers = String::from_utf8_lossy(buffer);
        let request: Vec<&str> = headers.split("\r\n\r\n").collect();
        let mut headers = request.get(0).unwrap().split("\r\n");
        let method = Method::from_string(&headers.next().unwrap().split(" ").nth(0).unwrap());

        for header in headers.into_iter() {
            let h = header.split_once(":");
            if h.is_some() {
                let (key, value) = h.unwrap();
                headers_map.insert(key.to_ascii_lowercase(), value.trim().to_owned());
            }
        }
        
        if method == Method::POST {
            // Check content-type 
            let default = "0".to_owned();
            let content_length = headers_map.get("content-length").unwrap_or(&default);
            let length = content_length.parse::<usize>().unwrap();
            let content = &request.get(1).unwrap().to_owned()[0..length];
            body = Some(content.to_owned());
        }

        Request { 
            headers: headers_map,
            params: HashMap::new(),
            method,
            body
        }
    }

    pub fn get_headers(&self) -> &Headers {
        &self.headers
    }

    pub fn get_header(&self, header: &str) -> Option<&String> {
        self.headers.get(&header.to_ascii_lowercase())
    }

    pub fn set_header(&mut self, header: &str, value: &str) {
        self.headers.insert(header.to_ascii_lowercase(), value.to_owned());
    }

    pub fn get_param(&self, param: &str) ->Option<&String> {
        self.params.get(param)
    }

    pub fn set_param(&mut self, param: &str, value: &str) {
        self.params.insert(param.to_owned(), value.to_owned());
    }

    pub fn get_method(&self) -> &Method {
        &self.method
    }

    pub fn get_body(&self) -> &Option<String> {
        &self.body
    }
}
