

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

    fn simple_logger() -> MiddlewareMutex {
        return new_middleware(|request| {
            println!("{:?}", request.method_and_path);
            (request, None)
        });
    } 
        

    add_route(&mut router, "GET /", Arc::clone(&handle_hello_world), vec![]);
    add_route(&mut router, "GET /log", Arc::clone(&handle_hello_world), vec![simple_logger()]);



    let listener = tokio::net::TcpListener::bind("127.0.0.1:8080").await;
    app::serve(router, listener).await; 
}
