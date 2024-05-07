

mod http;

use std::sync::Arc;

use http::app::serve;
use http::router::{Router, new_router, new_handler, add_route};
use http::handler::HandlerMutex;
use http::response::new_response;
use http::middleware::{new_middleware, MiddlewareMutex};

#[tokio::main]
async fn main() {


	let mut router: Router = new_router();

    let handle_hello_world: HandlerMutex = new_handler(|_| {
        return new_response(200, "Hello, World!".to_string());
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
    serve(router, listener).await; 

}
