

mod http;

use std::sync::Arc;

use http::router::{Router, Route, new_router, add_route, serve};
use http::handler::{HandlerMutex, new_handler};
use http::response::new_response;
use http::middleware::{mw_trace_init, mw_trace_log_request};

#[tokio::main]
async fn main() {


	let router: Router = new_router();

    let handle_hello_world: HandlerMutex = new_handler(|request| {
        return (request, new_response(200, "Hello, World!".to_string()));
    });

    let router = add_route(router, Route {
        path: "GET /hello",
        handler: Arc::clone(&handle_hello_world),
        middlewares: vec![mw_trace_init()],
        outerwares: vec![mw_trace_log_request()],
    });

    let router = add_route(router, Route {
        path: "GET /",
        handler: Arc::clone(&handle_hello_world),
        middlewares: vec![mw_trace_init()],
        outerwares: vec![mw_trace_log_request()],
    });

    
    // TODO: convert types to &str if possible
    serve(router, "127.0.0.1:8080".to_string()).await; 

}
