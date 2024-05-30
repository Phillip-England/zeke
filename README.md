# Zeke

A set of simple http primitives used to build web services, written in Rust.

## Quickstart

### Installation

In your `cargo.toml`:

```toml
[dependencies]
zeke = '0.1.3'
```

### Create a Router

Routers are used to define and serve our http endpoints.

```rs
#[tokio::main]
async fn main() {
	let r = Router::new();
}
```

### Create a Handler

Any function that returns a Handler can be associated with an endpoint:

```rs
#[tokio::main]
async fn main() {
	let r = Router::new();
    r.add(Route::new("GET /", hello_world()));
}

async fn hello_world() -> Handler {
    return Handler::new(|request| {
        // enables our handlers to by async
        Box::pin(async move {
            let response = Response::new()
                .status(200);
            return (request, response);
        })
    });
}
```

### Serving

To serve the application, called `Router.serve`:

```rs
#[tokio::main]
async fn main() {
    // --snip
	let result = r.serve(&host).await;
	if result.is_err() {
		println!("Error: {:?}", err);
	}
}
```

### Context Keys

Any data shared between middleware, handlers, and outerware is referred to as `context`.

Keys are required to encode and decode context. An enum which implements the `Contextable` trait can be used to keep track of these keys:

```rs
pub enum AppContext {
    Trace,
}

impl Contextable for AppContext {
    fn key(&self) -> &'static str {
        match self {
            AppContext::Trace => {"TRACE"},
        }
    }
}
```

### HttpTrace

HttpTrace is a `context` (because it is intended to be shared between middleware, handlers, and outware) that helps us keep track of how long each request cycle takes.

You must derive `Serialize` and `Deserialize` for any data intended to be used as `context`.

```rs
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

### Middleware

Any function that returns a `Middleware` can be used as middleware in our application.

Let's make use of the `HttpTrace` type we created in the previous section.

The following middleware will initialize `HttpTrace` prior to calling our handler:

```rs
pub async fn mw_trace() -> Middleware {
    Middleware::new(|mut request: &mut Request| {
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
        request.set_context(AppContext::Trace, trace_encoded);
        None
    })
}
```

Let's take a moment to notice a few key things going on here.

1. We initalize our trace type and then encode it into json:

```rs
let trace = HttpTrace{
    time_stamp: chrono::Utc::now().to_rfc3339(),
};
let trace_encoded = serde_json::to_string(&trace);
```

2. We ensure the trace has been encoded correctly:

```rs
if trace_encoded.is_err() {
    return Some(Response::new()
        .status(500)
        .body("failed to encode trace")
    );
}
```

3. Finally (and most importantly) we call set_context on our `Request` type, using our AppContext::Trace key

```rs
let trace_encoded = trace_encoded.unwrap();
request.set_context(AppContext::Trace, trace_encoded);
```

Now the json data for the `HttpTrace` type is associated with the `Request` type and can be used later in the request cycle.

We can attach our middleware to a `Route` like so:

```rs
#[tokio::main]
async fn main() {
	let r = Router::new();
    r.add(Route::new("GET /", hello_world())
        .middleware(mw_trace())
    );
    let result = r.serve(&host).await;
	if result.is_err() {
		println!("Error: {:?}", err);
	}
}
```

### Outerware

Any function that returns a `Middleware` can be used as outerware in our application. 

Middleware is ran *before* the handler is called.

Outerware is ran *after* the handler is called.

We can create an outerware to decode our `HttpTrace` type after the request cycle is over. We can then calculate how much time it took the entire request to process and print it to the terminal.

```rs
pub async fn mw_trace_log() -> Middleware {
    Middleware::new(|request: &mut Request | {
        let trace = request.get_context(AppContext::Trace);
        if trace.is_empty() {
            return Some(Response::new()
                .status(500)
                .body("failed to get trace")
            );
        }
        let trace: HttpTrace = serde_json::from_str(&trace).unwrap();
        let elapsed_time = trace.get_time_elapsed();
        let log_message = format!("[{:?}][{}][{}]", request.method, request.path, elapsed_time);
        println!("{}", log_message);
        None
    })
}
```

Let's take a closer look at a few things.

1. We use our `AppContext::Trace` key to get the encoded `HttpTrace` using `request.get_context`.

```rs
let trace = {
    let req = request.read().await;
    req.get_context(AppContext::Trace)
};
```

2. We ensure the trace exists:

```rs
if trace == "" {
    return Some(Response::new()
        .status(500)
        .body("failed to get trace")
    );
}
```

3. We decode the `HttpTrace`:

```rs
let trace: HttpTrace = serde_json::from_str(&trace).unwrap();
```

4. Finally, we calculate the elapsed time and log results to the terminal:

```rs
let elapsed_time = trace.get_time_elapsed();
let log_message = format!("[{:?}][{}][{}]", request.method, request.path, elapsed_time);
println!("{}", log_message);
```

We can use this outerware in our application like so:

```rs
#[tokio::main]
async fn main() {
	let r = Router::new();
    r.add(Route::new("GET /", hello_world())
        .middleware(mw_trace().await)
        .outerware(mw_trace_log().await)
    );
    let result = r.serve(&host).await;
	if result.is_err() {
		println!("Error: {:?}", err);
	}
}
```

### Middleware Groups

Any function that returns a `MiddlewareGroup` can be used as a middleware group in our application.

Middleware groups enable us to group middleware together. Let's see if we can group our `mw_trace` and `mw_trace_log` functions together:

```rs
pub fn mw_group_trace() -> MiddlewareGroup {
    return MiddlewareGroup::new(vec![mw_trace().await], vec![mw_trace_log().await]);
}
```

Now we can simply use the group:

```rs
#[tokio::main]
async fn main() {
	let r = Router::new();
    r.add(Route::new("GET /", hello_world())
        .group(mw_group_trace().await)
    );
    let result = r.serve(&host).await;
	if result.is_err() {
		println!("Error: {:?}", err);
	}
}
```

