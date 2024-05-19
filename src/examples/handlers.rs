
use crate::http::handler::Handler;
use crate::http::response::Response;

pub const NAVBAR: &str = "<nav><a href='/'>Home</a> | <a href='/about'>About</a> | <a href='/test/query_params?name=zeke&age=your mom'>Query Params</a></nav>";

pub fn base_template(title: &str) -> String {
    return format!(r#"
        <html>
            <head>
                <title>{}</title>
            </head>
            <body>
                {}
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