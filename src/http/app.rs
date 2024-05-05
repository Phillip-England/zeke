



use std::collections::HashMap;
use std::sync::Arc;

use crate::http::socket;
use crate::http::router::Router;



pub async fn serve(router: Arc<Router>) {
	loop {
		let listener = tokio::net::TcpListener::bind("127.0.0.1:8080").await;
		match listener {
			Ok(listener) => {
				let router = router.clone();
				socket::connect(listener, router).await;
			},
			Err(e) => {
				panic!("Error binding to address: {}", e);
			},
		}
	}
}


