

use crate::http::handler::Handler;
use crate::http::response::Response;


pub fn handle_home() -> Handler {
    return Handler::new(|request| {
        Box::pin(async move {
            let response = Response::new()
                .status(200)
                .body("Hello, World!");
            return (request, response);
        })
    });
}