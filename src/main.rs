

use std::sync::Arc;

use zeke::http::{
    router::{Router, Route},
    handler::{Handler, ArcHandler},
    response::{new_response, set_header},
    middleware::{MiddlewareMutex, HttpTrace, mw, mw_group, MiddlewareGroup},
    context::{get_context, set_context, ContextKey},
};


#[tokio::main]
async fn main() {


	let mut r = Router::new();

    pub fn handle_home() -> ArcHandler {
        return Handler::new(|request| {
            let response = new_response(200, "<h1>Home</h1>");
            let response = set_header(response, "Content-Type", "text/html");
            return (request, response);
        });
    }

    // // TODO: make it so handlers are stored in a func and returned
    // let handle_home: ArcHandler = 
    // });

    pub const KEY_TRACE: &ContextKey = "TRACE";

    pub fn mw_trace() -> MiddlewareMutex {
        return mw(|request| {
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
        return mw(|request| {
            let mw_trace = get_context(&request.context, KEY_TRACE.to_string());
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

    r.add(Route::new("GET /", handle_home())
        .middleware(mw_trace())
        .outerware(mw_trace_log())
    );

    pub fn mw_group_trace() -> MiddlewareGroup {
        return mw_group(vec![mw_trace()], vec![mw_trace_log()]);
    }

    r.add(Route::new("GET /about", handle_home())
        .group(mw_group_trace())
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
