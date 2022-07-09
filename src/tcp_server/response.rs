use std::fs;

use serde::Serialize;
use serde_json::json;

use crate::tcp_server::Headers;

pub struct Response {
    headers: Headers,
    content: Option<String>
}

impl Response {
    pub fn new(headers: Headers) -> Response {
        Response { 
            headers, 
            content: None
        }
    }

    pub fn json(&mut self, content: impl Serialize) {
        let json = json!(content);
        self.content = Some(serde_json::to_string_pretty(&json).unwrap());
        self.headers.insert("Content-Type".to_owned(), "application/json".to_owned());
        
    }

    pub fn text(&mut self, content: String) {
        self.content = Some(content);
    }

    pub fn html(&mut self, path: &str) {
        let contents = fs::read_to_string(path).unwrap(); 
        self.content = Some(contents);
    }

    pub fn get_content(&self) -> String {
        self.content.as_ref().unwrap().clone()
    }

    pub fn as_mut(&mut self) -> &mut Response {
        self
    }

    pub fn get_headers(&self) -> &Headers {
        &self.headers
    }

    pub fn set_header(&mut self, header: &str, value: &str) {
        self.headers.insert(header.to_owned(), value.to_owned());
    }
}