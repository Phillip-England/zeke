# Zeke

## Simple HTTP Library
Zeke is a HTTP library built on top of Tokio. Zeke values *simplicitiy and minimalism*.

## Quickstart
This quickstart will show you how to:
1. Create a Router
2. Add New Routes
3. Create Middleware
4. Apply Middleware to Routes
5. Share Context Between Middlware and Handlers

```rs

```


## Features

### Router
Initialize a server by creating a Router:

```rs
let mut r: Router = Router::new();
```

### Handlers
Handlers are functions that return a Handler:

```rs
pub fn handle_home() -> Handler {
    return Handler::new(|request| {
        let response = new_response(200, "<h1>Home</h1>");
        let response = set_header(response, "Content-Type", "text/html");
        return (request, response);
    });
}
```

### Routes
Routes are added to the Router:

```rs
r.add(Route::new("GET /", handle_home()))
```

### Middleware
Middleware is any function that returns a Middleware:

```rs
pub fn mw_print_name() -> Middleware {
    return Middleware::new(|request| {
        let name = "Zeke";
        println!("My name is {}", name);
        return None;
    })
}

pub fn mw_print_color() -> Middleware {
    return Middleware::new(|request| {
        let name = "red";
        println!("My favorite color is {}", name);
        return None;
    })
}
```

Middleware can be chained:
```rs
r.add(Route::new("GET /name-and-color", handle_hello_world())
    .middleware(mw_print_name(), mw_print_color())
);
```

Middleware can be grouped:
```rs
pub fn mw_group_name_and_color() -> MiddlewareGroup {
    return MiddlewareGroup::new(vec![mw_print_name()], vec![mw_print_color()]);
}

r.add(Route::new("GET /name-and-color-group", handle_hello_world())
    .group(mw_group_name_and_color())
);
```

If a Middlware returns a Response, the request cycle will end and the Response will be returned:
```rs
pub fn mw_stop_execution() -> Middleware {
    return Middleware::new(|request| {
        println!("response returned, stopping execution...");
        return Some(new_response(500, "stopping execution!"));
    })
}

r.add(Route::new("GET /stop", handle_hello_world())
    .middleware(mw_print_name(), mw_print_color())
);
```

### Outerware
Outerware are Middleware to be ran *after* processing a request:

```rs
r.add(Route::new("GET /name-then-color", handle_hello_world())
    .middleware(mw_print_name())
    .outerware(mw_print_color()) // accepts any Middleware
);
```

### Shared State

#### Define a Shared Type
Start by defining a type we intend to share between middleware, handlers, and outerware. Here, we define `HttpTrace` which will be used to log out request details after a request is processed.

NOTE: Any type we intend to share must derive `Serialize` and `Deserialize` from [serde](https://docs.rs/serde/latest/serde/index.html). I am using `version serde = { version = "1.0.200", features = ["derive"] }` in my `cargo.toml`.

```rs
use serde::{Deserialize, Serialize};

// -- snip

#[derive(Debug, Serialize, Deserialize)]
pub struct HttpTrace {
    pub time_stamp: String,
}

impl HttpTrace {
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
```

#### Define Your Context Keys
Context keys are used to encode and decode our shared types between each part of the request/response cycle.

Start by defining an enum containing our context keys:

```rs
pub enum AppContext {
    // define your keys here
    Trace
}
```

Implement the `Contextable` trait on our `AppContext` and list our keys:
```rs
impl Contextable for AppContext {
    fn key(&self) -> &'static str {
        match self {
            // list your keys here
            AppContext::Trace => {"TRACE"},
        }
    }
}
```

#### Encoding a Shared Type
Using `AppContext`, we can encode our shared types. Here we create a middleware which uses our `HttpTrace` type to track when we start processing our request:

```rs
pub fn mw_trace() -> Middleware {
    return Middleware::new(|request| {
        let trace = HttpTrace{
            time_stamp: chrono::Utc::now().to_rfc3339(),
        };
        let trace_encoded = serde_json::to_string(&trace);
        match trace_encoded {
            Ok(trace_encoded) => {
                // using our key to encode the HttpTrace
                set_context(request, AppContext::Trace, trace_encoded);
                return None;
            },
            Err(_) => {
                return Some(new_response(500, "failed to encode trace"));
            }
        }
    });
}
```

#### Decoding a Shared Type
Using `AppContext`, we can decode our shared types. Here, we create a middleware which will decode our `HttpTrace` and log out all the request details, including how long it took the request to process:

```rs
pub fn mw_trace_log() -> Middleware {
    return Middleware::new(|request| {
        let trace = get_context(&request.context, AppContext::Trace);
        if trace == "" {
            return Some(new_response(500, "trace not found"));
        }
        // decode our type here
        let trace: HttpTrace = serde_json::from_str(&trace).unwrap();
        let elapsed_time = trace.get_time_elapsed();
        let log_message = format!("[{}][{}][{}]", request.method, request.path, elapsed_time);
        println!("{}", log_message);
        return None;
    });
}
```



