
use std::sync::{Arc, Mutex};

use serde::{Deserialize, Serialize};

use crate::http::request::{Request, set_context_encoded, RequestContextKey};
use crate::http::response::Response;

pub type Middleware = Box<dyn Fn(&mut Request) -> Option<Response> + Send + 'static>;
pub type MiddlewareMutex = Arc<Mutex<Middleware>>;
pub type Middlewares = Vec<MiddlewareMutex>;

pub fn new_middleware<F>(f: F) -> MiddlewareMutex
where
	F: Fn(&mut Request) -> Option<Response> + Send + 'static,
{
	Arc::new(Mutex::new(Box::new(f)))    
}

pub const KEY_TRACE: &str = "TRACE";

pub fn mw_trace() -> MiddlewareMutex {
    return new_middleware(|request| {
        set_context_encoded(request, KEY_TRACE.to_string(), HttpTrace{
            time_stamp: chrono::Utc::now().to_rfc3339(),
        });
        return None;
    });
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HttpTrace {
    pub time_stamp: String,
}

