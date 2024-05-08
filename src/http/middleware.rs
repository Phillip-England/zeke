
use std::sync::{Arc, Mutex};

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::http::request::{Request, RequestContextKey};
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

pub const KEY_TRACE: &RequestContextKey = "TRACE";
#[derive(Debug, Serialize, Deserialize)]
pub struct HttpTrace {
    pub time_stamp: String,
}

impl HttpTrace {
    /// Prints the time elapsed since the `time_stamp` was set.
    pub fn get_time_elapsed(&self) -> String {
        // Parse the stored RFC3339 timestamp back into a DateTime<Utc>
        if let Ok(time_set) = DateTime::parse_from_rfc3339(&self.time_stamp) {
            let time_set = time_set.with_timezone(&Utc);

            // Get the current UTC time
            let now = Utc::now();

            // Calculate the duration elapsed
            let duration = now.signed_duration_since(time_set);
            let micros = duration.num_microseconds();
            match micros {
                Some(micros) => {
                    if micros < 1000 {
                        return format!("{}µ", micros);
                    }
                },
                None => {

                }
            }
            let millis = duration.num_milliseconds();
            return format!("{}ms", millis);
        } else {
            return "failed to parse time_stamp".to_string();
        }
    }
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
        if mw_trace == "" {
            return Some(new_response(500, "trace not found".to_string()));
        }
        let trace: HttpTrace = serde_json::from_str(&mw_trace).unwrap();
        let elapsed_time = trace.get_time_elapsed();
        let log_message = format!("[{}] - [{}] - [{}]", request.method, request.path, elapsed_time);
        println!("{}", log_message);

        return None;
    });
}


