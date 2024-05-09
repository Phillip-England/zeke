

mod http;

use std::sync::Arc;

use http::router::{Router, Route};
use http::handler::{HandlerMutex, new_handler};
use http::response::{new_response, set_header};
use http::middleware::{new_middleware, MiddlewareMutex, HttpTrace};
use http::request::{extract_context_str, set_context, RequestContextKey};

#[tokio::main]
async fn main() {


	let mut r = Router::new();

    let handle_hello_world: HandlerMutex = new_handler(|request| {
        let response = new_response(200, "<h1>Hello, World!</h1>");
        let response = set_header(response, "Content-Type", "text/html");
        return (request, response);
    });

    pub const KEY_TRACE: &RequestContextKey = "TRACE";

    pub fn mw_trace() -> MiddlewareMutex {
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
                    return Some(new_response(500, "failed to encode trace"));
                }
            }
        });
    }
    
    pub fn mw_trace_log() -> MiddlewareMutex {
        return new_middleware(|request| {
            let mw_trace = extract_context_str(&request.context, KEY_TRACE.to_string());
            if mw_trace == "" {
                return Some(new_response(500, "trace not found"));
            }
            let trace: HttpTrace = serde_json::from_str(&mw_trace).unwrap();
            let elapsed_time = trace.get_time_elapsed();
            let log_message = format!("[{}][{}][{}]", request.method, request.path, elapsed_time);
            println!("{}", log_message);
    
            return None;
        });
    }

    r.add(Route::new("GET /", handle_hello_world)
        .middleware(mw_trace())
        .outerware(mw_trace_log())
    );

    
    // TODO: convert types to &str if possible
    let err = r.serve("127.0.0.1:8080").await;
    match err {
        Some(e) => {
            println!("Error: {:?}", e);
        },
        None => {
            println!("Server closed");
        },
    }



}
