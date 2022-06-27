use serde::Serialize;
use serde_json::json;

use crate::http_server::Headers;

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
    }

    pub fn get_content(self) -> String {
        self.content.unwrap().to_owned()
    }

    pub fn as_mut(&mut self) -> &mut Response {
        self
    }
}