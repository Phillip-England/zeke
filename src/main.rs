use std::{io::Write, net::{TcpListener, TcpStream}, thread, time::Duration};

use http::request;
mod http;


fn main() {


    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
	let pool = http::pool::ThreadPool::new(4);

    for stream in listener.incoming() {
        let mut stream = stream.unwrap();
		pool.execute(move || {
			let request = http::request::Request::new(&stream);
			let response = handle_connection(request);
			stream.write(response.as_bytes()).unwrap();
		})
    }


}

fn handle_connection(request: http::request::Request) -> String {

	if request.path_and_method == "GET /" {
		let response = http::response::hello_world();
		return response;
	}

	if request.path_and_method == "GET /sleep" {
		thread::sleep(Duration::from_secs(5));
		let response = http::response::hello_world();
		return response;
	}

	// not found
	let response = http::response::not_found();
	return response;


	// parsing the request

 
}
