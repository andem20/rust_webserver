use serde_json::{Error};

pub trait Response {
    fn get_value(&self) -> Result<String, Error>;
}