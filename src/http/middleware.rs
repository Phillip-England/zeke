
use std::sync::{Arc, Mutex};

use serde::{Deserialize, Serialize};

use crate::http::request::{Request, set_context_encoded, RequestContextKey};
use crate::http::response::{new_response, Response};

use super::request::{extract_context_str, set_context};

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
#[derive(Debug, Serialize, Deserialize)]
pub struct HttpTrace {
    pub time_stamp: String,
}

pub fn mw_trace_init() -> MiddlewareMutex {
    return new_middleware(|request| {
        let trace = HttpTrace{
            time_stamp: chrono::Utc::now().to_rfc3339(),
        };
        let trace_encoded = serde_json::to_string(&trace);
        match trace_encoded {
            Ok(trace_encoded) => {
                set_context(request, KEY_TRACE.to_string(), trace_encoded);
                return None;
            },
            Err(_) => {
                return Some(new_response(500, "failed to encode trace".to_string()));
            }
        }
    });
}

pub fn mw_trace_log_request() -> MiddlewareMutex {
    return new_middleware(|request| {
        let mw_trace = extract_context_str(&request.context, KEY_TRACE.to_string());
        println!("TRACE: {}", mw_trace);
        if mw_trace == "" {
            return Some(new_response(500, "trace not found".to_string()));
        }
        let trace: HttpTrace = serde_json::from_str(&mw_trace).unwrap();
        println!("TRACE: {}", trace.time_stamp);
        return None;
    });
}


