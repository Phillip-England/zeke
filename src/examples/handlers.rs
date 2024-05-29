

use crate::http::handler::Handler;
use crate::http::response::Response;


pub fn handle_home() -> Handler {
    return Handler::new(|request| {
        let response = Response::new()
            .status(200);
        return (request, response);
    });
}