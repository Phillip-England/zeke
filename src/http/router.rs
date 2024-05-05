use std::{collections::HashMap, sync::{Arc, Mutex}};

pub type Router = HashMap<&'static str, Arc<Mutex<Box<dyn Fn() + Send + 'static>>>>;

pub fn new_router() -> Router {
	let router: Router = HashMap::new();
	return router
}

pub fn insert(router: &mut Router, path: &'static str, handler: Box<dyn Fn() + Send + 'static>) {
	router.insert(path, Arc::new(Mutex::new(handler)));
}