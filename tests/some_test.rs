mod test_data;
mod setup_teardown;


#[test]
pub fn web_server_test() {
    let expected = test_data::get_data();

    let mut server = setup_teardown::setup();

    let request = reqwest::blocking::get("http://localhost:8080");
    let body = request.unwrap().text().unwrap();
    
    let actual = serde_json::from_str::<test_data::TestDTO>(body.as_str()).unwrap();
    
    assert!(actual == expected);

    server.close();
}