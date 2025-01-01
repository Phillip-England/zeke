use crate::route::Route;
use crate::handler::Handler;

use hyper::Request;


#[derive(Debug)]
pub struct Router<'a> {
    routes: std::collections::HashMap<String, Route<'a>>,
}

impl<'a> Router<'a> {
    pub fn new() -> Self {
        Self {
            routes: std::collections::HashMap::new(),
        }
    }

    pub fn add<F>(&mut self, method: &'a str, path: &'a str, handler: F)
    where
        F: Fn(Request<hyper::body::Incoming>) + 'a + 'static + Send + Sync,
    {
        let boxed_handler: Handler = Box::new(handler);
        let route = Route {
            path,
            method,
            handler: boxed_handler,
        };
        self.routes.insert(path.to_string(), route);
    }
}