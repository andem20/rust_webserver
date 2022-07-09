use std::collections::HashMap;

use super::{request::Request, response::Response};

type Handler = fn(req: &Request, res: &mut Response);

#[derive(Clone)]
pub struct Route {
    endpoint: String,
    handler: Option<Handler>,
    method: Option<Method>,
    routes: HashMap<String, Route>,
}

impl Route {
    pub fn new(endpoint: &str, handler: Option<Handler>, method: Option<Method>) -> Route {
        Route {
            endpoint: endpoint.to_owned(),
            handler,
            method,
            routes: HashMap::new(),
        }
    }

    pub fn get(endpoint: &str, handler: Handler) -> Route {
        Route::new(endpoint, Some(handler), Some(Method::GET))
    }

    pub fn get_endpoint(&self) -> &String {
        &self.endpoint
    }

    pub fn get_handler(&self) -> &Option<Handler> {
        &self.handler
    }

    pub fn get_method(&self) -> &Option<Method> {
        &self.method
    }

    pub fn get_routes(&self) -> &HashMap<String, Route> {
        &self.routes
    }

    pub fn add_route(&mut self, key: &str, value: Route) {
        self.routes.insert(key.to_owned(), value);
    }

    pub fn as_mut(&mut self) -> &mut Route {
        self
    }

    pub fn get_route(&mut self, key: &str) -> Option<&mut Route> {
        self.routes.get_mut(key)
    }

    pub fn set_handler(&mut self, handler: Handler) {
        self.handler = Some(handler);
    }

    pub fn set_method(&mut self, method: Method) {
        self.method = Some(method);
    }
}

#[derive(Clone)]
pub enum Method {
    GET,
    POST,
}
