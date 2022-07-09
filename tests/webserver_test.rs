use serde_json::json;

mod util;

use util::{test_data, handlers, setup_teardown};
use webserver::tcp_server::route::Route;

const URL: &'static str = "http://localhost";

#[test]
fn web_server_test() {
    let expected = test_data::get_data();

    let routes = vec![
        Route::get("/", handlers::index_handler),
    ];
    
    let port = 10000;

    let mut server = setup_teardown::setup(routes, port);

    let url = format!("{}:{}", URL, port);

    let request = reqwest::blocking::get(url);
    let body = request.unwrap().text().unwrap();
    
    let actual = serde_json::from_str::<test_data::TestDTO>(body.as_str()).unwrap();
    
    assert!(actual == expected);

    server.close();
}

#[test]
fn params_test() {
    let routes = vec![
        Route::get("/users/:id", handlers::users_handler),
    ];

    let port = 10001;

    let mut server = setup_teardown::setup(routes, port);

    let expected = "12";

    let url = format!("{}:{}/users/{}", URL, port, expected);

    let request = reqwest::blocking::get(url);
    let body = request.unwrap().text().unwrap();
    
    let actual = body.as_str();

    let expected = json!(expected).to_string();

    assert!(actual == expected);

    server.close();
}