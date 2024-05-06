use std::{collections::HashMap, sync::{Arc, Mutex}};

use crate::http::response::Response;
use crate::http::request::Request;
use crate::http::middleware::{Middlewares, MiddlewareMutex};

pub type RouteHandler = (Handler, Middlewares);
pub type RouteHandlerMutex = Arc<Mutex<RouteHandler>>;
pub type Router = HashMap<&'static str, Arc<Mutex<RouteHandler>>>;
pub type Handler = Arc<Mutex<Box<dyn Fn(Request) -> Response + Send + 'static>>>;



pub fn new_router() -> Router {
	let router: Router = HashMap::new();
	return router
}

pub fn add_route(router: &mut Router, path: &'static str, handler: Handler, middlewares: Middlewares) {
	let handler: RouteHandler = (handler, middlewares);
    let handler_mutex = Arc::new(Mutex::new(handler));
	router.insert(path, handler_mutex);
}


pub fn new_handler<F>(f: F) -> Handler
where
    F: Fn(Request) -> Response + Send + 'static,
{
    Arc::new(Mutex::new(Box::new(f)))
}


