



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
	log.reset_log(Logs::HttpTest);
	log.reset_log(Logs::Debug);

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


	let c_log = log.clone();
	let test_task = tokio::spawn(async {
        test(host.to_string(), c_log).await;
    });

	let c_log = log.clone();
	let server_task = tokio::spawn(async move {
		match r.serve(host, c_log).await {
			Some(e) => {
				log.log(Logs::ServerError, &format!("error running Router.serve(): {:?}", e));
				println!("Error: {:?}", e)
			},
			None => println!("Server closed"),
		}
	});

	let _ = tokio::join!(test_task, server_task);

}









