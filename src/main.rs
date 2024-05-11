


use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use zeke::http::{
    router::{Router, Route},
    handler::Handler,
    response::{new_response, set_header},
    middleware::{Middleware, MiddlewareKey, MiddlewareGroup},
    context::{get_context, set_context, ContextKey, Contextable},
};


#[tokio::main]
async fn main() {

    // initialize a new router
	let mut r = Router::new();

    // mount a handler with middleware/outerware
    r.add(Route::new("GET /", handle_home())
        .middleware(mw_trace().1)
        .outerware(mw_trace_log().1)
    );

    // mount a handler with a middleware group
    r.add(Route::new("GET /about", handle_home())
        .group(mw_group_trace())
    );

    // start the server
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

// creating a handler
pub fn handle_home() -> Handler {
    return Handler::new(|request| {
        let response = new_response(200, "<h1>Home</h1>");
        let response = set_header(response, "Content-Type", "text/html");
        return (request, response);
    });
}

// user defined context
pub enum AppContext {
    Trace,
}

// implementing Contextable for AppContext
impl Contextable for AppContext {
    fn to_key(&self) -> &'static str {
        match self {
            AppContext::Trace => {
                return "TRACE";
            }
        }
    }
}

// creating a middleware
pub fn mw_trace() -> (MiddlewareKey, Middleware) {
    return Middleware::new("TRACE",|request| {
        let trace = HttpTrace{
            time_stamp: chrono::Utc::now().to_rfc3339(),
        };
        let trace_encoded = serde_json::to_string(&trace);
        match trace_encoded {
            Ok(trace_encoded) => {
                let (key, _) = mw_trace();
                set_context(request, AppContext::Trace, trace_encoded);
                return None;
            },
            Err(_) => {
                return Some(new_response(500, "failed to encode trace"));
            }
        }
    });
}

// creating another middleware
pub fn mw_trace_log() -> (MiddlewareKey, Middleware) {
    return Middleware::new("", |request| {
        let trace = get_context(&request.context, AppContext::Trace);
        if trace == "" {
            return Some(new_response(500, "trace not found"));
        }
        let trace: HttpTrace = serde_json::from_str(&trace).unwrap();
        let elapsed_time = trace.get_time_elapsed();
        let log_message = format!("[{}][{}][{}]", request.method, request.path, elapsed_time);
        println!("{}", log_message);
        return None;
    });
}

// grouping middleware
pub fn mw_group_trace() -> MiddlewareGroup {
    let (_, mw_trace) = mw_trace();
    let (_, mw_trace_log) = mw_trace_log();
    return MiddlewareGroup::new(vec![mw_trace], vec![mw_trace_log]);
}

// a type to store a timescamp in our context
// Deserialize is required for the `get_context` function to work on the type
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

