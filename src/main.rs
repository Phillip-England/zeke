



use zeke::http::logger::{Logger, Logs};
use zeke::http::router::{Route, Router};

use zeke::examples::{
    handlers::{handle_home, handle_about, handle_query_params, handle_post_with_body, handle_put, handle_delete},
    middleware::mw_group_trace,
};

use zeke::tests::test::test;


#[tokio::main]
async fn main() {

    dotenv::dotenv().ok();

	let log = Logger::new();
	log.reset_log(Logs::Trace);
	log.reset_log(Logs::ServerError);
	log.log(Logs::Trace, "spawned in main.rs");
	

    let host = "127.0.0.1:8080";
	let mut r = Router::new();

    r.add(Route::new("GET /", handle_home())
        .group(mw_group_trace())
    );

    r.add(Route::new("GET /test/query_params", handle_query_params())
        .group(mw_group_trace())
    );

    r.add(Route::new("POST /test/post_with_body", handle_post_with_body())
        .group(mw_group_trace())
    );

    r.add(Route::new("GET /about", handle_about())
        .group(mw_group_trace())
    );

    r.add(Route::new("PUT /test/put", handle_put())
        .group(mw_group_trace())
    );

    r.add(Route::new("DELETE /test/delete", handle_delete())
    	.group(mw_group_trace())
	);

	log.log(Logs::Trace, "creating thread for test functions");
    let test_task = tokio::spawn(async {
        test(host.to_string()).await;
    });

	log.log(Logs::Trace, "creating thread for server");
    let server_task = tokio::spawn(async move {
        match r.serve(host, log).await {
            Some(e) => println!("Error: {:?}", e),
            None => println!("Server closed"),
        }
    });

    let _ = tokio::join!(test_task, server_task);

}









