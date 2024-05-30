use std::sync::Arc;
use tokio::sync::RwLock;
use futures::future::BoxFuture;

use crate::http::request::Request;
use crate::http::response::Response;

pub type MiddlewareFunc = dyn Fn(Arc<RwLock<Request>>) -> BoxFuture<'static, Option<Response>> + Send + Sync + 'static;

pub struct Middleware {
    pub func: Arc<RwLock<Box<MiddlewareFunc>>>,
}

impl Middleware {
    pub fn new<F>(f: F) -> Middleware
    where
        F: Fn(Arc<RwLock<Request>>) -> BoxFuture<'static, Option<Response>> + Send + Sync + 'static,
    {
        Middleware {
            func: Arc::new(RwLock::new(Box::new(f))),
        }
    }
}


pub type Middlewares = Vec<Middleware>;

pub struct MiddlewareGroup {
    pub middlewares: Middlewares,
    pub outerwares: Middlewares,
}

impl MiddlewareGroup {
    pub fn new(middlewares: Middlewares, outerwares: Middlewares) -> MiddlewareGroup {
        MiddlewareGroup {
            middlewares,
            outerwares,
        }
    }
}
