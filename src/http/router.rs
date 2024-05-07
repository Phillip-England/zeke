use std::{collections::HashMap, sync::{Arc, Mutex}};

use crate::http::middleware::Middlewares;
use crate::http::handler::HandlerMutex;

pub type RouteHandler = (HandlerMutex, Middlewares);
pub type Router = HashMap<&'static str, Arc<Mutex<RouteHandler>>>;
pub struct Route {
    pub path: &'static str,
    pub handler: HandlerMutex,
    pub middlewares: Middlewares,
}

pub fn new_router() -> Router {
	let router: Router = HashMap::new();
	return router
}

pub fn add_route(mut router: Router, route: Route) -> Router {
	let handler: RouteHandler = (route.handler, route.middlewares);
    let handler_mutex = Arc::new(Mutex::new(handler));
	router.insert(route.path, handler_mutex);
    return router;
}


