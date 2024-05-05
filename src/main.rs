use std::{io::Write, net::{TcpListener, TcpStream}};
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
	println!("{:?}", request.raw.lines);
	println!("{:?}", request.raw.method_line());
	println!("{:?}", request.raw.host_line());
	println!("{:?}", request.raw.agent_line());

	// parsing the request

    let response = http::response::hello_world();
    stream.write(response.as_bytes()).unwrap();
}
