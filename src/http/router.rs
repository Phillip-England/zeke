use std::{collections::HashMap, sync::{Arc, Mutex}};

use crate::http::response::Response;
use crate::http::request::Request;

pub type Router = HashMap<&'static str, Arc<Mutex<Box<dyn Fn(Request) -> Response + Send + 'static>>>>;
pub type Handler = Box<dyn Fn(Request) -> Response + Send + 'static>;

pub fn new_router() -> Router {
	let router: Router = HashMap::new();
	return router
}

pub fn insert(router: &mut Router, path: &'static str, handler: Handler) {
	router.insert(path, Arc::new(Mutex::new(handler)));
}

pub fn create_handler<F>(f: F) -> Handler
where
    F: Fn(Request) -> Response + Send + 'static,
{
    Box::new(f)
}

