# Zeke

## Simple HTTP Library
Zeke is a HTTP library built on top of Tokio. Zeke values *simplicitiy and minimalism*.

## Features

### Router
Initialize a server by creating a Router:

```rs
let mut r: Router = Router::new();
```

### Handlers
Handlers are functions that return a Handler:

```rs
fn handle_hello_world() -> Handler {
    return Handler::new(|request| {
        let response = new_response(200, "<h1>Hello, World!</h1>");
        let response = set_header(response, "Content-Type", "text/html");
        return (request, response);
    });
}
```

### Routes
Routes are added to the Router type:

```rs
r.add(Route::new("GET /", handle_hello_world()))
```

### Middleware
Middleware is any function that returns a Middleware:

```rs
pub fn mw_print_name() -> Middleware {
    return Middleware::new(|request| {
        let name = "Zeke";
        println!("My name is {}", name);
    })
}

pub fn mw_print_color() -> Middleware {
    return Middleware::new(|request| {
        let name = "red";
        println!("My favorite color is {}", name);
    })
}
```

Middleware can be chained:
```rs
r.add(Route::new("GET /name-and-color", handle_hello_world())
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

Start by defining a type you intend to share between middleware, handlers, and outerware. Here, we define `HttpTrace` which will be used to log out request details after a request is processed.

NOTE: Any type you intend to share must derive `Serialize` and `Deserialize` from [serde](https://docs.rs/serde/latest/serde/index.html). I am using `version serde = { version = "1.0.200", features = ["derive"] }` in my `cargo.toml`.

```rs
use serde::{Deserialize, Serialize};

// -- snip

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
```

Context keys can be created and used to share data of any type between our middleware, handlers, and outerware:

Start by defining an enum containing your keys:

```rs
// you can call this whatever you want
pub enum AppContext {
    TraceKey
}
```

Implement the `Contextable` trait on your `AppContext`:
```rs
impl Contextable for AppContext {
    fn key(&self) -> &'static str {
        match self {
            // defining the &str associated our AppContext enum
            AppContext::TraceKey => {"TRACE"},
            // other keys go here..
        }
    }
}
```




