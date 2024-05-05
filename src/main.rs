use std::{io::Write, net::{TcpListener, TcpStream}, thread, time::Duration};
mod http;


fn main() {


    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
	let pool = http::pool::ThreadPool::new(4);

    for stream in listener.incoming() {
        let stream = stream.unwrap();
		pool.execute(|| {
			handle_connection(stream);
		})
    }


}

fn handle_connection(mut stream: TcpStream) {
    let request = http::request::Request::new(&stream);
	println!("{:?}", request.method);
	println!("{:?}", request.path);

	if request.method == "GET" && request.path == "/" {
		let response = http::response::hello_world();
		stream.write(response.as_bytes()).unwrap();
		return;
	}

	if request.method == "GET" && request.path == "/sleep" {
		thread::sleep(Duration::from_secs(5));
		let response = http::response::hello_world();
		stream.write(response.as_bytes()).unwrap();
		return;
	}

	// not found
	let response = http::response::not_found();
	stream.write(response.as_bytes()).unwrap();


	// parsing the request

 
}
