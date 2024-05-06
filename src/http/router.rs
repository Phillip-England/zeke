use std::{collections::HashMap, sync::{Arc, Mutex}};

use crate::http::response::Response;
use crate::http::request::Request;

pub type RouteHandler = (Handler, Middlewares);
pub type RouteHandlerMutex = Arc<Mutex<RouteHandler>>;
pub type Router = HashMap<&'static str, Arc<Mutex<RouteHandler>>>;

pub type Middleware = Box<dyn Fn(Request) -> (Request, Option<Response>)  + Send + 'static>;

pub type MiddlewareMutex = Arc<Mutex<Middleware>>;
pub type Middlewares = Vec<MiddlewareMutex>;

pub fn new_router() -> Router {
	let router: Router = HashMap::new();
	return router
}

pub fn insert(router: &mut Router, path: &'static str, handler: Handler, middlewares: Middlewares) {
	let handler: RouteHandler = (handler, middlewares);
    let handler_mutex = Arc::new(Mutex::new(handler));
	router.insert(path, handler_mutex);
}

pub type Handler = Box<dyn Fn(Request) -> Response + Send + 'static>;

pub fn new_handler<F>(f: F) -> Handler
where
    F: Fn(Request) -> Response + Send + 'static,
{
    Box::new(f)
}


pub fn new_middleware<F>(f: F) -> MiddlewareMutex
where
	F: Fn(Request) -> (Request, Option<Response>) + Send + 'static,
{
	Arc::new(Mutex::new(Box::new(f)))
}

pub fn test1_middleware() -> MiddlewareMutex {
	new_middleware(|request: Request| {
		let response = Some(Response {
			status: 200,
			body: "I am first!".to_string(),
		});
        return (request, response);
	})
}

pub fn test_middleware() -> MiddlewareMutex {
	new_middleware(|request: Request| {
		let response = Some(Response {
			status: 200,
			body: "Hello, Middleware!".to_string(),
		});
        return (request, response);
	})
}