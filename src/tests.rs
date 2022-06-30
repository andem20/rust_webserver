
#[cfg(test)]
mod tests {
    use serde::{Serialize, Deserialize};
    
    use crate::{route::Route, http_server::HTTPServer, request::Request, response::Response};
    
    #[derive(Serialize, Deserialize, PartialEq, Eq)]
    struct TestDTO {
        name: String,
        age: u8,
        phones: Vec<String>
    }
    
    fn test_handler(req: &Request, res: &mut Response) {
        let data = TestDTO {
            name: "John Doe".to_owned(),
            age: 43,
            phones: [
                "+44 1234567".to_owned(),
                "+44 2345678".to_owned()
            ].to_vec()
        };
    
        res.json(data);
    }
    
    fn setup() -> HTTPServer {
        let mut server = HTTPServer::new("127.0.0.1", 8080, 4);
    
        // Set up all endpoints
        server.routes(vec![
            Route::get("/", test_handler),
        ]);

        server.listen(|this| {
            println!("Now listening on http://{}:{}", 
                this.get_host(), 
                this.get_port()
            );
        });

        server
    }
    
    #[test]
    fn web_server_test() {
        let expected = TestDTO {
            name: "John Doe".to_owned(),
            age: 43,
            phones: [
                "+44 1234567".to_owned(),
                "+44 2345678".to_owned()
            ].to_vec()
        };

        let mut server = setup();

        let request = reqwest::blocking::get("http://localhost:8080");
        let body = request.unwrap().text().unwrap();
        
        let actual = serde_json::from_str::<TestDTO>(&body).unwrap();
        
        assert!(actual == expected);

        server.close();
    }
}