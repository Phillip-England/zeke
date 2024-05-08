

mod http;

use std::sync::Arc;

use http::router::{Router, Route, new_router, add_route, serve};
use http::handler::{HandlerMutex, new_handler};
use http::response::{new_response, set_header};
use http::middleware::{mw_trace_init, mw_trace_log_request};

#[tokio::main]
async fn main() {


	let router: Router = new_router();

    let handle_hello_world: HandlerMutex = new_handler(|request| {
        let response = new_response(200, "<h1>Hello, World!</h1>".to_string());
        return (request, new_response(200, "<h1>Hello, World!</h1>".to_string()));
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
    serve(router, "127.0.0.1:8080").await; 

}
