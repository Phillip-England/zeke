use std::{io::Write, net::{TcpListener, TcpStream}, thread, time::Duration};

mod http;

use http::app::App;
use http::pool::ThreadPool;
use http::request::Request;


fn main() {


	// binding to a port and getting a thread pool
	// thread pool will ensure we can handle multiple requests at the same time
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
	let pool = ThreadPool::new(4);

	// setting up our app routes
	let mut app = App::new();

	app.route("GET /", Box::new(|_request| {
		return "HTTP/1.1 200 OK\r\n\r\nHello, World!".to_string();
	}));

    for stream in listener.incoming() {
        let stream = stream.unwrap();
		pool.execute(move || {
			handle_connection(stream);
		})
    }


}

fn handle_connection(mut stream: TcpStream, app: App) -> () {

	let request = Request::new(&stream);

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
