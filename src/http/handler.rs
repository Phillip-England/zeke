
use std::sync::{Arc, Mutex};

use crate::http::request::Request;
use crate::http::response::Response;

pub type Handler = Box<dyn Fn(Request) -> Response + Send + 'static>;
pub type HandlerMutex = Arc<Mutex<Box<dyn Fn(Request) -> Response + Send + 'static>>>;

pub fn new_handler<F>(f: F) -> HandlerMutex
where
    F: Fn(Request) -> Response + Send + 'static,
{
    Arc::new(Mutex::new(Box::new(f)))
}
