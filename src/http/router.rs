use std::{collections::HashMap, sync::{Arc, Mutex}};

use crate::http::response::Response;

pub type Router = HashMap<&'static str, Arc<Mutex<Box<dyn Fn() -> Response + Send + 'static>>>>;

pub fn new_router() -> Router {
	let router: Router = HashMap::new();
	return router
}

pub fn insert(router: &mut Router, path: &'static str, handler: Box<dyn Fn() -> Response + Send + 'static>) {
	router.insert(path, Arc::new(Mutex::new(handler)));
}