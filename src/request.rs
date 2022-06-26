use std::collections::HashMap;

type Headers = HashMap<String, String>;
pub struct Request {
    headers: Headers,
}

impl Request {
    pub fn new(buffer: &[u8]) -> Request {
        let mut request = Request { 
            headers: HashMap::new()
        };

        let headers = String::from_utf8_lossy(buffer);
        let headers = headers.split("\r\n");

        for header in headers.into_iter() {
            let h = header.split_once(": ");
            if h.is_some() {
                let (key, value) = h.unwrap();
                request.headers.insert(key.to_owned(), value.to_owned());
            }
        }

        request
    }

    pub fn get_headers(&self) -> &Headers {
        &self.headers
    }
}
