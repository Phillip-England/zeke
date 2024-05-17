
use crate::http::handler::Handler;
use crate::http::response::Response;

pub fn handle_home() -> Handler {
    return Handler::new(|request| {
        let response = Response::new()
            .status(200)
            .body("<h1>Home</h1><a href='/about'>About</a>")
            .set_header("Content-Type", "text/html")
            .set_header("Zeke", "zeke and his mom rule!")
            .set_header("Zekes-Mom", "so does zeke's mom");
        return (request, response);
    });
}

pub fn handle_about() -> Handler {
    return Handler::new(|request| {
        let response = Response::new()
            .status(200)
            .body("<h1>About</h1><a href='/'>Home</a>")
            .set_header("Content-Type", "text/html");
        return (request, response);
    });
}