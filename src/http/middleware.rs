
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::http::request::Request;
use crate::http::response::Response;

pub type MiddlewareFunc = dyn Fn(&mut Request) -> Option<Response> + Send + Sync + 'static;
pub type Middleware = Arc<Mutex<MiddlewareFunc>>;
pub type Middlewares = Vec<Middleware>;

pub struct MiddlewareGroup {
    pub middlewares: Middlewares,
    pub outerwares: Middlewares,
}

pub fn mw<F>(f: F) -> Middleware
where
	F: Fn(&mut Request) -> Option<Response> + Send + Sync + 'static,
{
	Arc::new(Mutex::new(Box::new(f)))    
}

pub fn mw_group(middlewares: Vec<Middleware>, outerwares: Vec<Middleware>) -> MiddlewareGroup {
    return MiddlewareGroup {
        middlewares: middlewares,
        outerwares: outerwares,
    };
}






