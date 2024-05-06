
use std::sync::{Arc, Mutex};

use crate::http::request::Request;
use crate::http::response::Response;

pub type Middleware = Box<dyn Fn(Request) -> (Request, Option<Response>)  + Send + 'static>;
pub type MiddlewareMutex = Arc<Mutex<Middleware>>;
pub type Middlewares = Vec<MiddlewareMutex>;

pub fn new_middleware<F>(f: F) -> MiddlewareMutex
where
	F: Fn(Request) -> (Request, Option<Response>) + Send + 'static,
{
	Arc::new(Mutex::new(Box::new(f)))
}