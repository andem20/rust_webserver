use crate::{response::Response, request::Request};

type Handler = fn(req: &Request, res: &mut Response);

pub struct Route {
    endpoint: String,
    handler: Handler
}

impl Route {
    pub fn get(endpoint: &str, handler: Handler) -> Route {
        Route {
            endpoint: endpoint.to_string(),
            handler,
        }
    }

    pub fn get_endpoint(&self) -> &String {
        &self.endpoint
    }

    pub fn get_handler(&self) -> &Handler {
        &self.handler
    }
}