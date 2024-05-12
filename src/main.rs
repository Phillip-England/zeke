



use zeke::http::router::{Route, Router};

use zeke::examples::{
    handlers::{handle_home, handle_about},
    middleware::mw_group_trace,
};

use zeke::tests::http::http_test;


#[tokio::main]
async fn main() {


    let host = "127.0.0.1:8080";
	let mut r = Router::new();

    r.add(Route::new("GET /", handle_home())
        .group(mw_group_trace())
    );

    r.add(Route::new("GET /about", handle_about())
        .group(mw_group_trace())
    );

    // Spawn a new task for the http_test function
    let http_test_task = tokio::spawn(async {
        http_test(host).await;
    });

    // Spawn the server as another task
    let server_task = tokio::spawn(async move {
        match r.serve(host).await {
            Some(e) => println!("Error: {:?}", e),
            None => println!("Server closed"),
        }
    });

    // Use tokio::join! to wait for both tasks to complete
    let _ = tokio::join!(http_test_task, server_task);

}









