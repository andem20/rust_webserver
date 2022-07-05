use rand::Rng;
use webserver::route::Route;

mod util;

use util::{test_data, handlers, setup_teardown};

const URL: &'static str = "http://localhost";

#[test]
fn web_server_test() {
    let expected = test_data::get_data();

    let routes = vec![
        Route::get("/", handlers::index_handler),
    ];
    
    let mut rng = rand::thread_rng();

    let port = rng.gen_range(10000..20000);

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

    let mut rng = rand::thread_rng();

    let port = rng.gen_range(10000..20000);

    let mut server = setup_teardown::setup(routes, port);

    let expected = "12";

    let url = format!("{}:{}", URL, port);

    let request = reqwest::blocking::get(url);
    let body = request.unwrap().text().unwrap();
    
    let actual = body.as_str();

    assert!(actual == expected);

    server.close();
}