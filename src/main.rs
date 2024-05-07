

mod http;

use std::sync::Arc;

use http::router::{Router, Route, new_router, add_route, serve};
use http::handler::{HandlerMutex, new_handler};
use http::response::new_response;
use http::middleware::{new_middleware, MiddlewareMutex};

#[tokio::main]
async fn main() {


	let router: Router = new_router();

    let handle_hello_world: HandlerMutex = new_handler(|_| {
        return new_response(200, "Hello, World!".to_string());
    });

    fn simple_logger() -> MiddlewareMutex {
        return new_middleware(|request| {
            println!("{:?}", request.method_and_path);
            (request, None)
        });
    } 
        
    let router = add_route(router, Route {
        path: "GET /hello",
        handler: Arc::clone(&handle_hello_world),
        middlewares: vec![simple_logger()],
    });

    let router = add_route(router, Route {
        path: "GET /",
        handler: Arc::clone(&handle_hello_world),
        middlewares: vec![simple_logger()],
    });


    serve(router, "127.0.0.1:8080".to_string()).await; 

}
