use crate::{response::Response, IndexDTO};

type Handler = fn() -> Box<dyn Response>;

pub struct Route {
    endpoint: String,
    handler: Handler
}

impl Route {
    pub fn get<T: Response>(endpoint: &str, handler: Handler) -> Route {
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