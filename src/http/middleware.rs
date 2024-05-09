
use std::sync::Arc;
use tokio::sync::Mutex;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::http::request::Request;
use crate::http::response::Response;


pub type Middleware = Box<dyn Fn(&mut Request) -> Option<Response> + Send + 'static>;
pub type MiddlewareMutex = Arc<Mutex<Middleware>>;
pub type Middlewares = Vec<MiddlewareMutex>;


pub struct MiddlewareGroup {
    pub middlewares: Middlewares,
    pub outerwares: Middlewares,
}

impl MiddlewareGroup {
    pub fn new(middlewares: Vec<MiddlewareMutex>, outerwares: Vec<MiddlewareMutex>) -> MiddlewareGroup {
        return MiddlewareGroup {
            middlewares: middlewares,
            outerwares: outerwares,
        };
    }
}

pub fn new_middleware<F>(f: F) -> MiddlewareMutex
where
	F: Fn(&mut Request) -> Option<Response> + Send + 'static,
{
	Arc::new(Mutex::new(Box::new(f)))    
}

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




