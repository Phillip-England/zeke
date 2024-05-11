# Zeke

## HTTP Library
Zeke is a HTTP library built on top of Tokio. Zeke values *simplicity*. Things like routing, setting up handlers, and chaining middleware should be easy.

## Features

### Router
Initialize a server by creating a Router:

```rs
let mut r: Router = Router::new();
```

### Handlers
Handlers are functions that *return* a Handler:

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
Routes are *added* to the Router type:

```rs
r.add(Route::new("GET /", handle_hello_world()))
```


