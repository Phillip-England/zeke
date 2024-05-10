# Zeke

## HTTP Library
Zeke is a HTTP library built on top of Tokio. Zeke values *simplicity*. Things like routing, setting up handlers, and chaining middleware should be easy.

## Minimal
Zeke aims to stay out of your way. 

## Quickstart

init a router:
```rs
let mut r: Router = Router::new();
```

create a handler:
```rs
fn handle_home() -> Handler {
    return Handler::new(|request| {
        let response = new_response(200, "<h1>Home</h1>");
        let response = set_header(response, "Content-Type", "text/html");
        return (request, response);
    });
}
```

add the handler to your router:
```rs
r.add(Route::new("GET /", handle_home()));
```

And all together we have:
```rs

use zeke::http::{
    router::Router,
    handler::Handler,
    response::{new_response, set_header},
};

fn main() {

    let mut r: Router = Router::new();

    fn handle_home() -> Handler {
        return Handler::new(|request| {
            let response = new_response(200, "<h1>Home</h1>");
            let response = set_header(response, "Content-Type", "text/html");
            return (request, response);
        });
    }

    r.add(Route::new("GET /", handle_home()));

}
```


