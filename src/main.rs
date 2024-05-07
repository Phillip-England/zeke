

mod http;

use std::sync::Arc;

use http::router::{Router, Route, new_router, add_route, serve};
use http::handler::{HandlerMutex, new_handler};
use http::response::new_response;
use http::middleware::{new_middleware, MiddlewareMutex};
use http::request::{set_context, extract_context_str};
use serde::{Deserialize, Serialize};

#[tokio::main]
async fn main() {


	let router: Router = new_router();

    let handle_hello_world: HandlerMutex = new_handler(|_| {
        return new_response(200, "Hello, World!".to_string());
    });

    #[derive(Debug, Serialize, Deserialize)]
    pub struct Person {
        name: String,
    }
    

    fn middleware_set_context() -> MiddlewareMutex {
        return new_middleware(|request| {
            set_context(request, "hi".to_string(), "hi".to_string());
            let person: Person = Person {
                name: "Phillip".to_string(),
            };
            let person_as_str = serde_json::to_string(&person);
            match person_as_str {
                Ok(person) => {
                    set_context(request, "person".to_string(), person);
                    return None;
                }
                Err(e) => {
                    return Some(new_response(500, format!("failed to parse json into string: {}", e)))
                }
            }
        });
    } 

    fn middleware_get_context() -> MiddlewareMutex {
        return new_middleware(|request| {
            let person = extract_context_str(&request.context, "person".to_string());
            println!("{}", person);
            return None;
        });
    } 

        
    let router = add_route(router, Route {
        path: "GET /hello",
        handler: Arc::clone(&handle_hello_world),
        middlewares: vec![middleware_set_context(), middleware_get_context()],
    });

    let router = add_route(router, Route {
        path: "GET /",
        handler: Arc::clone(&handle_hello_world),
        middlewares: vec![middleware_set_context(), middleware_get_context()],

    });


    serve(router, "127.0.0.1:8080".to_string()).await; 

}
