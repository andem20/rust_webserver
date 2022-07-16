use std::fs;

use serde::Serialize;

use super::headers_type::Headers;

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
        let json = serde_json::to_string(&content).unwrap();
        println!("{}", json);
        self.content = Some(json);
        self.headers.insert("content-type".to_owned(), "application/json".to_owned());
        
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
        self.headers.insert(header.to_ascii_lowercase(), value.to_owned());
    }
}