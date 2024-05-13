



use std::env;

use zeke::http::router::{Route, Router};

use zeke::examples::{
    handlers::{handle_home, handle_about},
    middleware::mw_group_trace,
};

use zeke::tests::http::http_test;


#[tokio::main]
async fn main() {

    dotenv::dotenv().ok();

    let host = match env::var("TEST_HOST") {
        Ok(value) => value,
        Err(_) => "127.0.0.1:8080".to_string(),
    };

    let host = "127.0.0.1:8080";
	let mut r = Router::new();

    r.add(Route::new("GET /", handle_home())
        // .group(mw_group_trace())
    );

    r.add(Route::new("GET /about", handle_about())
        // .group(mw_group_trace())
    );

    let http_test_task = tokio::spawn(async {
        http_test(host.to_string()).await;
    });

    let server_task = tokio::spawn(async move {
        match r.serve(host).await {
            Some(e) => println!("Error: {:?}", e),
            None => println!("Server closed"),
        }
    });

    let _ = tokio::join!(http_test_task, server_task);

}









