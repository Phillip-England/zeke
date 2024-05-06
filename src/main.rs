

mod http;

use std::sync::Arc;

use http::app;
use http::router;

#[tokio::main]
async fn main() {


	let mut router: router::Router = router::new_router();

    router::insert(&mut router, "/", Box::new(|| {
        http::response::Response {
            status: 200,
            body: "Hello, World!".to_string(),
        }
	}));

	let router = Arc::new(router);

    app::serve(router).await;    
}
