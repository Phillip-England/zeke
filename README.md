# Zeke

## HTTP Library
Zeke is a HTTP library built on top of Tokio. Zeke values *simplicity*. Things like routing, setting up handlers, and chaining middleware should be easy.

## Minimal
Zeke aims to stay out of your way. 

## Quickstart

```rs
fn main() {

    // init a router
    let mut r: Router = Router::new();

    // create a handler
    fn handle_home() -> Handler {
        return Handler::new(|request| {
            let response = new_response(200, "<h1>Home</h1>");
            let response = set_header(response, "Content-Type", "text/html");
            return (request, response);
        });
    }

    // mount the handler
    r.add(Route::new("GET /", handle_home()));

    // serve
    let err = r.serve("127.0.0.1:8080").await;
    match err {
        Some(e) => {
            println!("error: {:?}", e);
        },
        None => {
            println!("server closed");
        },
    }

}
```


