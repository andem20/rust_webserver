use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
pub struct TestDTO {
    name: String,
    age: u8,
    phones: Vec<String>
}

pub fn get_data() -> TestDTO {
    TestDTO {
        name: "John Doe".to_owned(),
        age: 43,
        phones: [
            "+44 1234567".to_owned(),
            "+44 2345678".to_owned()
        ].to_vec()
    }
}
