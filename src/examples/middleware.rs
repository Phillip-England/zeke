
use std::sync::Arc;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use futures::future::BoxFuture;
use futures::FutureExt;
use tokio::sync::RwLock;

use crate::http::request::Request;
use crate::MiddlewareGroup;
use crate::{http::middleware::Middleware, Response};
use crate::examples::context::AppContext;

pub async fn mw_trace() -> Middleware {
    Middleware::new(|request: Arc<RwLock<Request>>| {
        async move {
            let trace = HttpTrace {
                time_stamp: chrono::Utc::now().to_rfc3339(),
            };
            let trace_encoded = serde_json::to_string(&trace);
            if trace_encoded.is_err() {
                return Some(Response::new()
                    .status(500)
                    .body("failed to encode trace")
                );
            }
            let trace_encoded = trace_encoded.unwrap();
            {
                let mut req = request.write().await;
                req.set_context(AppContext::Trace, trace_encoded);
            }
            None
        }.boxed()
    })
}

pub async fn mw_trace_log() -> Middleware {
    Middleware::new(|request: Arc<RwLock<Request>>| {
        async move {
            let trace = {
                let req = request.read().await;
                req.get_context(AppContext::Trace)
            };
            if trace.is_empty() {
                return Some(Response::new()
                    .status(500)
                    .body("failed to get trace")
                );
            }
            let trace: HttpTrace = serde_json::from_str(&trace).unwrap();
            let elapsed_time = trace.get_time_elapsed();
            {
                let req = request.read().await;
                let log_message = format!("[{:?}][{}][{}]", req.method, req.path, elapsed_time);
                println!("{}", log_message);
            }
            None
        }.boxed()
    })
}

pub async fn mw_group_trace() -> MiddlewareGroup {
    MiddlewareGroup::new(vec![mw_trace().await], vec![mw_trace_log().await])
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