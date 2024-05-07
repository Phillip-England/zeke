

mod http;

use std::sync::Arc;

use http::router::{Router, Route, new_router, add_route, serve};
use http::handler::{HandlerMutex, new_handler};
use http::response::new_response;
use http::middleware::{new_middleware, MiddlewareMutex};
use http::request::{set_context, extract_context_str};

#[tokio::main]
async fn main() {


	let router: Router = new_router();

    let handle_hello_world: HandlerMutex = new_handler(|_| {
        return new_response(200, "Hello, World!".to_string());
    });

    fn middleware_set_context() -> MiddlewareMutex {
        return new_middleware(|request| {
            println!("{:?}", request.method_and_path);
            set_context(request, "hi".to_string(), "hi".to_string());
            return None;
        });
    } 

    fn middleware_get_context() -> MiddlewareMutex {
        return new_middleware(|request| {
            let hi = extract_context_str(&request.context, "hi".to_string());
            println!("{}", hi);
            return None;
        });
    } 

        
    let router = add_route(router, Route {
        path: "GET /hello",
        handler: Arc::clone(&handle_hello_world),
        middlewares: vec![middleware_set_context(), middleware_get_context()],
    });

    let router = add_route(router, Route {
        path: "GET /",
        handler: Arc::clone(&handle_hello_world),
        middlewares: vec![middleware_set_context(), middleware_get_context()],

    });


    serve(router, "127.0.0.1:8080".to_string()).await; 

}
