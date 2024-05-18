
use crate::http::handler::Handler;
use crate::http::response::Response;

pub const Navbar: &str = "<nav><a href='/'>Home</a> | <a href='/about'>About</a> | <a href='/test/query_params'>Query Params</a></nav>";

pub fn handle_home() -> Handler {
    return Handler::new(|request| {
        let response = Response::new()
            .status(200)
            .body(Navbar)
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
            .body(Navbar)
            .set_header("Content-Type", "text/html");
        return (request, response);
    });
}

pub fn handle_query_params() -> Handler {
    return Handler::new(|request| {
        let response = Response::new()
            .status(200)
            .body(Navbar)
            .set_header("Content-Type", "text/html");
        return (request, response);
    });
}