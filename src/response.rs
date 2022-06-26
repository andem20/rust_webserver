use serde_json::{Value, Error};

pub trait Response {
    fn get_value(&self) -> Result<String, Error>;
}