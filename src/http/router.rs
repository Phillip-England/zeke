use std::{collections::HashMap, sync::{Arc, Mutex}};
use std::io::Error;

use crate::http::middleware::Middlewares;
use crate::http::handler::HandlerMutex;
use crate::http::socket::connect_socket;

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

pub async fn serve(router: Router, addr: String) -> Option<Error> {
    let listener = tokio::net::TcpListener::bind(addr).await;
    let router: Arc<Router> = Arc::new(router);
    match listener {
        Ok(ref listener) => {
            loop {
                let router: Arc<Router> = Arc::clone(&router); // TODO: is cloning the router bad?
                connect_socket(listener, router).await; 
            }
        },
        Err(e) => {
			return Some(e);
        },
    }
}


