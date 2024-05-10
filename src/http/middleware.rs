
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::http::request::Request;
use crate::http::response::Response;

pub type Middleware = dyn Fn(&mut Request) -> Option<Response> + Send + Sync + 'static;
pub type MiddlewareMutex = Arc<Mutex<Middleware>>;
pub type Middlewares = Vec<MiddlewareMutex>;

pub struct MiddlewareGroup {
    pub middlewares: Middlewares,
    pub outerwares: Middlewares,
}

pub fn mw<F>(f: F) -> MiddlewareMutex
where
	F: Fn(&mut Request) -> Option<Response> + Send + Sync + 'static,
{
	Arc::new(Mutex::new(Box::new(f)))    
}

pub fn mw_group(middlewares: Vec<MiddlewareMutex>, outerwares: Vec<MiddlewareMutex>) -> MiddlewareGroup {
    return MiddlewareGroup {
        middlewares: middlewares,
        outerwares: outerwares,
    };
}






