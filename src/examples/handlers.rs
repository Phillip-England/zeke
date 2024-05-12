
use crate::http::handler::Handler;
use crate::http::response::Response;

pub fn handle_home() -> Handler {
    return Handler::new(|request| {
        let response = Response::new(200, "<h1>Home</h1><a href='/about'>About</a>")
            .set_header("Content-Type", "text/html");
        return (request, response);
    });
}

pub fn handle_about() -> Handler {
    return Handler::new(|request| {
        let response = Response::new(200, "<h1>About</h1><a href='/'>Home</a>")
            .set_header("Content-Type", "text/html");
        return (request, response);
    });
}