
use tokio::sync::Mutex;
use std::sync::Arc;


use crate::http::request::Request;
use crate::http::response::Response;

pub type HandlerFunc = Arc<Box<dyn Fn(Request) -> (Request, Response) + Send + 'static>>;
// pub type HandlerFunc = Box<dyn Fn(Request) -> (Request, Response) + Send + 'static>;

pub struct Handler {
    pub func: Arc<Mutex<Box<dyn Fn(Request) -> (Request, Response) + Send + 'static>>>,
} 

pub type ArcHandler = Arc<Handler>;

impl Handler {
    pub fn new<F>(f: F) -> ArcHandler
    where
        F: Fn(Request) -> (Request, Response) + Send + 'static,
    {
        Arc::new(Handler {
            func: Arc::new(Mutex::new(Box::new(f)))
        })
    }
    pub fn clone(&self) -> Handler {
        Handler {
            func: Arc::clone(&self.func)
        }
    }
}