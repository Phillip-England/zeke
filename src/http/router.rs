use std::{collections::HashMap, sync::{Arc, Mutex}};
use std::io::Error;

use crate::http::middleware::Middlewares;
use crate::http::handler::HandlerMutex;
use crate::http::socket::connect_socket;

pub type RouteHandler = (HandlerMutex, Middlewares, Middlewares);

pub type Routes = HashMap<&'static str, Arc<Mutex<RouteHandler>>>;

pub struct Router {
    pub routes: Routes,
}

impl Router {
    pub fn new() -> Router {
        Router {
            routes: HashMap::new(),
        }
    }
    pub fn add_route(self: &mut Router, route: Route) {
        let handler: RouteHandler = (route.handler, route.middlewares, route.outerwares);
        let handler_mutex = Arc::new(Mutex::new(handler));
        self.routes.insert(route.path, handler_mutex);
    }
}

pub struct Route {
    pub path: &'static str,
    pub handler: HandlerMutex,
    pub middlewares: Middlewares,
    pub outerwares: Middlewares,
}



pub async fn serve(router: Router, addr: &str) -> Option<Error> {
    let listener = tokio::net::TcpListener::bind(&addr).await;
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


