
use time::{Duration, OffsetDateTime};

use crate::http::cookie::Cookie;
use crate::http::handler::Handler;
use crate::http::response::Response;

pub const NAVBAR: &str = "<nav><a href='/'>Home</a> | <a href='/about'>About</a> | <a href='/test/query_params?name=zeke&age=your mom'>Query Params</a> | <a href='/test/set_cookie'>Set Cookies</a></nav>";

pub fn base_template(title: &str) -> String {
    return format!(r#"
        <html>
            <head>
                <title>{}</title>
            </head>
            <body>
                {}
                <h1>Hello, World</h1>
            </body>
        </html>
    "#, title, NAVBAR);
}


pub fn handle_home() -> Handler {
    return Handler::new(|request| {
        let response = Response::new()
            .status(200)
            .body(&base_template("Home"))
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
            .body(&base_template("About"))
            .set_header("Content-Type", "text/html");
        return (request, response);
    });
}

pub fn handle_query_params() -> Handler {
    return Handler::new(|request| {
        let response = Response::new()
            .status(200)
            .body(&base_template("Query Params"))
            .set_header("Content-Type", "text/html");
        return (request, response);
    });
}

pub fn handle_post_with_body() -> Handler {
    return Handler::new(|request| {
        let response = Response::new()
            .status(200)
            .body(&base_template("Post With Body"))
            .set_header("Content-Type", "text/html");
        return (request, response);
    });
}

pub fn handle_put() -> Handler {
    return Handler::new(|request| {
        let response = Response::new()
            .status(200)
            .body(&base_template("Basic Put"))
            .set_header("Content-Type", "text/html");
        return (request, response);
    });
}

pub fn handle_delete() -> Handler {
    return Handler::new(|request| {
        let response = Response::new()
            .status(200)
            .body(&base_template("Basic Delete"))
            .set_header("Content-Type", "text/html");
        return (request, response);
    });
}

pub fn handle_set_cookie() -> Handler {
	return Handler::new(|request| {
        println!("COOKIES: {:?}", request.cookies);
        let zekes_mom_cookie = request.get_cookie("zekes mom");
        println!("Zekes Mom Cookie: {:?}", zekes_mom_cookie);
		let response = Response::new()
			.status(200)
			.body(&base_template("Set Cookie"))
			.set_header("Content-Type", "text/html")
			.set_cookie(
                Cookie::new("zeke", "likes cookies")
                    .expires(OffsetDateTime::now_utc() + Duration::days(1))
                    .domain("")
                    .path("/")
                    .secure(false)
                    .http_only(false)
            )
            .set_cookie(
                Cookie::new("zekes mom", "likes cookies too")
                    .expires(OffsetDateTime::now_utc() + Duration::days(1))
                    .domain("")
                    .path("/")
                    .secure(false)
                    .http_only(false)
            );
		return (request, response);
	});
}