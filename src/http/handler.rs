
use tokio::sync::RwLock;
use std::sync::Arc;


use crate::http::request::Request;
use crate::http::response::Response;

pub struct Handler {
    pub func: Arc<RwLock<dyn Fn(Request) -> (Request, Response) + Send + Sync + 'static>>,
} 

impl Handler {
    pub fn new<F>(f: F) -> Handler
    where
        F: Fn(Request) -> (Request, Response) + Send + Sync + 'static,
    {
        Handler {
            func: Arc::new(RwLock::new(f))
        }
    }
}