
use tokio::sync::Mutex;
use std::sync::Arc;


use crate::http::request::Request;
use crate::http::response::Response;

pub type HandlerFunc = Box<dyn Fn(Request) -> (Request, Response) + Send + 'static>;
pub type Handler = Arc<Mutex<Box<dyn Fn(Request) -> (Request, Response) + Send + 'static>>>;

pub fn new_handler<F>(f: F) -> Handler
where
    F: Fn(Request) -> (Request, Response) + Send + 'static,
{
    Arc::new(Mutex::new(Box::new(f)))
}
