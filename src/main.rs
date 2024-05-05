use std::{io::Write, net::{TcpListener, TcpStream}};





mod http;


fn main() {


    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
	let pool = http::thread::ThreadPool::new(4);

    for stream in listener.incoming() {
        let stream = stream.unwrap();
		pool.execute(|| {
			handle_connection(stream);
		})
    }


}

fn handle_connection(mut stream: TcpStream) {
    println!("{:?}", stream);
    let http_request = http::request::stream_to_request_vector(&stream);
    let response = http::response::hello_world();
    stream.write(response.as_bytes()).unwrap();
    println!("{:?}", http_request)
}
