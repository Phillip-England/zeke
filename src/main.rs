



use zeke::http::router::{Route, Router};

use zeke::examples::{
    handlers::{handle_home, handle_about},
    middleware::mw_group_trace,
};


#[tokio::main]
async fn main() {

	let mut r = Router::new();

    r.add(Route::new("GET /", handle_home())
        .group(mw_group_trace())
    );

    // mount a handler with a middleware group
    r.add(Route::new("GET /about", handle_about())
        .group(mw_group_trace())
    );

    let err = r.serve("127.0.0.1:8080").await;
    match err {
        Some(e) => {
            println!("Error: {:?}", e);
        },
        None => {
            println!("Server closed");
        },
    }

}









