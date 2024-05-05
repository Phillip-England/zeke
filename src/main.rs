

mod http;

use std::sync::Arc;

use http::app;
use http::router;

#[tokio::main]
async fn main() {


	let mut router: router::Router = router::new_router();

    router::insert(&mut router, "/", Box::new(|| {
		println!("Hello from /");
	}));

	let arc_router = Arc::new(router);

    app::serve(arc_router).await;    
}
