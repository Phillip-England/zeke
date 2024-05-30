




use zeke::http::router::{Route, Router};

use zeke::examples::{
    handlers::{handle_home},
    middleware::mw_group_trace,
};

#[tokio::main]
async fn main() {

    dotenv::dotenv().ok();

    let host = "127.0.0.1:8080";
	let mut r = Router::new();

    r.add(Route::new("GET /", handle_home())
        .group(mw_group_trace().await)
    );

	let err = r.serve(&host).await;
	if err.is_err() {
		println!("Error: {:?}", err);
	}
	

}









