

mod http;

use std::sync::Arc;

use http::app;
use http::router::{Router, Handler, new_router, new_handler, add_route};

use http::middleware::{new_middleware, MiddlewareMutex};

#[tokio::main]
async fn main() {


	let mut router: Router = new_router();

    let handle_hello_world: Handler = new_handler(|request| {
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
        

    add_route(&mut router, "GET /", Arc::clone(&handle_hello_world), vec![custom_mw()]);
    add_route(&mut router, "GET /yo", Arc::clone(&handle_hello_world), vec![]);



    let listener = tokio::net::TcpListener::bind("127.0.0.1:8080").await;
    app::serve(router, listener).await; 
}
