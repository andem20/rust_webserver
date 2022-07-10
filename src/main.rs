
use std::{thread, time::Duration};

use webserver::{tcp_server::{tcp_server::TCPServer, route::Route}, handlers::{index_handler, slow_handler}};



fn main() {
    let mut server = TCPServer::new("127.0.0.1", 8080, 4);

    // Set up all endpoints
    server.routes(vec![
        // Route::get("/:id", index_handler),
        Route::get("/index/hej", index_handler),
        Route::get("/index/:id", index_handler),
        Route::get("/index/:id/newpath", index_handler),
        Route::get("/index/subroute/newpath", index_handler),
        Route::get("/slow", slow_handler),
        Route::get("/index", index_handler),
    ]);

    server.listen(|this| {
        println!("Now listening on http://{}:{}", 
            this.get_host(), 
            this.get_port()
        );
    });

    thread::sleep(Duration::from_millis(100000));
}