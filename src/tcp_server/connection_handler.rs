use std::{net::TcpStream, io::{Read, Write}, collections::HashMap, sync::Arc};

use serde_json::json;

use crate::tcp_server::{request::Request, response::Response, method::Method};

use super::route::Route;

pub fn connection_handler(mut stream: TcpStream, routes_map: Arc<HashMap<Method, Route>>) {
    let mut buffer = [0; 1024];

    stream.read(&mut buffer).unwrap();

    let headers = String::from_utf8_lossy(&buffer);

    let mut request = Request::new(&buffer);
    let mut response = Response::new(HashMap::new());
    
    let endpoint = headers.split(" ").nth(1).unwrap();

    let url = endpoint.split("/");
    let method = request.get_method();

    let mut route = routes_map.get(method).unwrap().clone();
    let mut valid = true;
    
    for branch in url {
        let key = format!("/{}", branch);
        if route.get_routes().contains_key(&key) {
            route = route.get_routes().get(&key).unwrap().clone();
            continue;
        }

        if route.get_routes().contains_key("/:") {
            // Remove leading slash
            let param = &route.get_routes().get("/:").unwrap().get_endpoint()[2..];
            println!("param: {}", param);
            request.set_param(param, branch);
            route = route.get_routes().get("/:").unwrap().clone();
            continue;
        }
        
        valid = false;
        break;
    }

    let (status, content) = if !valid {
        let value = serde_json::to_string(&json!({
            "error": "Page not found"
        })).unwrap();

        (404, value)
    } else {
        let handler = route.get_handler().unwrap();
        handler(&request, response.as_mut());
        let value = response.get_content();

        (200, value)
    };

    let mut headers = String::new();

    for header in response.get_headers() {
        let header = format!("{}: {}\r\n", header.0, header.1);
        headers.push_str(&header);
    }

    let response = format!(
        "HTTP/1.1 {}\r\nContent-Length: {}\r\n{}\r\n{}",
        status,
        &content.len(),
        &headers,
        &content
    );

    // println!("{}", &response);

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}