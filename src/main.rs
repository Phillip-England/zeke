

mod http;

use std::sync::Arc;

use http::app;
use http::request::get_header;
use http::router;


#[tokio::main]
async fn main() {


	let mut router: router::Router = router::new_router();

    let handle_hello_world = router::create_handler(|request| {
        let (request, header) = get_header(request, "Content-Type");
        println!("Content-Type: {}", header);
        http::response::Response {
            status: 200,
            body: "Hello, World!".to_string(),
        }
    });

    router::insert(&mut router, "GET /", handle_hello_world);

	let router = Arc::new(router);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:8080").await;
    app::serve(router, listener).await; 
}
