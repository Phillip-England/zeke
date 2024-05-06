

mod http;

use std::sync::Arc;

use http::app;
use http::router::{self, new_middleware, test1_middleware, test_middleware, Handler};

use crate::http::router::MiddlewareMutex;


#[tokio::main]
async fn main() {


	let mut router: router::Router = router::new_router();

    let handle_hello_world: Handler = router::new_handler(|request| {
        http::response::Response {
            status: 200,
            body: "Hello, World!".to_string(),
        }
    });

    fn custom_mw() -> MiddlewareMutex {
        return new_middleware(|request| {
            println!("{:?}", request.method_and_path);
            (request, None)
        });
    } 
        

    router::insert(&mut router, "GET /", Arc::clone(&handle_hello_world), vec![custom_mw()]);
    router::insert(&mut router, "GET /yo", Arc::clone(&handle_hello_world), vec![]);



    let listener = tokio::net::TcpListener::bind("127.0.0.1:8080").await;
    app::serve(router, listener).await; 
}
