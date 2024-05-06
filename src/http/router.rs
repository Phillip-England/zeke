use std::{collections::HashMap, sync::{Arc, Mutex}};

use crate::http::response::Response;
use crate::http::request::Request;

pub type RouteHandler = (Handler, Middlewares);
pub type RouteHandlerMutex = Arc<Mutex<RouteHandler>>;
pub type Router = HashMap<&'static str, RouteHandler>;
pub type Middleware = Box<dyn Fn(Request) -> Option<Response>>;
pub type Middlewares = Vec<Middleware>;

pub fn new_router() -> Router {
	let router: Router = HashMap::new();
	return router
}

pub fn insert(router: &mut Router, path: &'static str, handler: Handler, middlewares: Middlewares) {
	let handler: RouteHandler = (handler, middlewares);
	router.insert(path, handler);
}

pub type Handler = Box<dyn Fn(Request) -> Response + Send + 'static>;

pub fn new_handler<F>(f: F) -> Handler
where
    F: Fn(Request) -> Response + Send + 'static,
{
    Box::new(f)
}


pub fn new_middleware<F>(f: F) -> Middleware
where
	F: Fn(Request) -> Option<Response> + 'static,
{
	Box::new(f)
}

pub fn test_middleware() -> Middleware {
	new_middleware(|request: Request| {
		Some(Response {
			status: 200,
			body: "Hello, Middleware!".to_string(),
		})
	})
}