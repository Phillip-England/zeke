
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::MiddlewareGroup;
use crate::{http::middleware::Middleware, Response};
use crate::examples::context::AppContext;

pub fn mw_trace() -> Middleware {
    return Middleware::new(|request| {
        let trace = HttpTrace{
            time_stamp: chrono::Utc::now().to_rfc3339(),
        };
        let trace_encoded = serde_json::to_string(&trace);
        match trace_encoded {
            Ok(trace_encoded) => {
                request.set_context(AppContext::Trace, trace_encoded);
                return None;
            },
            Err(_) => {
                return Some(Response::new()
                    .status(500)
                    .body("failed to encode trace")
                );
            }
        }
    });
}

pub fn mw_trace_log() -> Middleware {
    return Middleware::new(|request| {
        let trace = request.get_context(AppContext::Trace);
        if trace == "" {
            return Some(Response::new()
                .status(500)
                .body("failed to get trace")
            );
        }
        let trace: HttpTrace = serde_json::from_str(&trace).unwrap();
        let elapsed_time = trace.get_time_elapsed();
        let log_message = format!("[{:?}][{}][{}]", request.method, request.path, elapsed_time);
        println!("{}", log_message);
        return None;
    });
}

pub fn mw_group_trace() -> MiddlewareGroup {
    return MiddlewareGroup::new(vec![mw_trace()], vec![mw_trace_log()]);
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HttpTrace {
    pub time_stamp: String,
}

impl HttpTrace {
    /// Prints the time elapsed since the `time_stamp` was set.
    pub fn get_time_elapsed(&self) -> String {
        if let Ok(time_set) = DateTime::parse_from_rfc3339(&self.time_stamp) {
            let time_set = time_set.with_timezone(&Utc);
            let now = Utc::now();
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