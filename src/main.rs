use std::{io::Write, net::{TcpListener, TcpStream}, sync::Arc, thread, time::Duration};

mod http;

use http::app::App;
use http::pool::ThreadPool;
use http::request::Request;


fn main() {


	// binding to a port and getting a thread pool
	// thread pool will ensure we can handle multiple requests at the same time
let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
let pool = ThreadPool::new(4);

// initializing the app
let mut app = App::new();

// adding routes
app.add_route("GET /", |request| {
	return http::response::hello_world();
});

// wrapping app in arc
let app = Arc::new(app);





for stream in listener.incoming() {
	let stream = stream.unwrap();
	let app = app.clone();
	pool.execute(move || {
		handle_connection(stream, app);
	})
}

}

fn handle_connection(mut stream: TcpStream, app: Arc<App>) -> () {

	let request = Request::new(&stream);
	let handler = app.get_handler(&request.path_and_method).unwrap().lock().unwrap()(request);


	if request.path_and_method == "GET /" {
		let response = http::response::hello_world();
		stream.write(response.as_bytes()).unwrap();
		return;
	}

	if request.path_and_method == "GET /sleep" {
		thread::sleep(Duration::from_secs(5));
		let response = http::response::hello_world();
		stream.write(response.as_bytes()).unwrap();
		return;
	}

	// not found
	let response = http::response::not_found();
	stream.write(response.as_bytes()).unwrap();
	return;

 
}
