

pub fn hello_world() -> String {
    return "HTTP/1.1 200 OK\r\n\r\nHello, world!".to_string();
}

pub fn not_found() -> String {
	return "HTTP/1.1 404 NOT FOUND\r\n\r\nNot Found".to_string();
}