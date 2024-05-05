
use std::{
    io::{prelude::*, BufReader},
    net::TcpStream,
};

#[derive(Debug)]
pub struct Request {
	pub raw: RawRequest,
	pub method: String,
	pub path: String,
	pub full_path: String,
	pub path_and_method: String,
}

impl Request {
	pub fn new(stream: &TcpStream) -> Request {
		let raw = RawRequest::new(stream);
		let (method, full_path, path_without_params) = raw.parse_method_and_path();
		let path_and_method = format!("{} {}", method, path_without_params);
		return Request { 
			raw: raw,
			method: method,
			path: path_without_params,
			full_path: full_path,
			path_and_method: path_and_method,
		};
	}
}

#[derive(Debug)]
pub struct RawRequest {
	pub lines: Vec<String>,
	method_line_index: usize,
	host_line_index: usize,
	agent_line_index: usize,
	accept_line_index: usize,
}

impl RawRequest {
	pub fn new(stream: &TcpStream) -> RawRequest {
		let lines = RawRequest::stream_to_lines(stream);
		RawRequest { 
			lines: lines,
			method_line_index: 0,
			host_line_index: 1,
			agent_line_index: 2,
			accept_line_index: 3,
		}

	}
	pub fn stream_to_lines(stream: &TcpStream) -> Vec<String> {
		let buf_reader: BufReader<&TcpStream> = BufReader::new(&stream);
		let http_request: Vec<_> = buf_reader
			.lines()
			.map(|result| result.unwrap())
			.take_while(|line| !line.is_empty())
			.collect();
		http_request
	}
	pub fn method_line(&self) -> &str {
		&self.lines[self.method_line_index]
	}

	/// Parses out the method and path from the method line.
	/// Returns a tuple containing the method, full path (with query params), and the request without any params.
	pub fn parse_method_and_path(&self) -> (String, String, String) {
		let method_line = self.method_line();
		let parts: Vec<&str> = method_line.split_whitespace().collect();
		let method = parts[0].to_string();
		let full_path = parts[1].to_string();
		let path_without_params = full_path.split("?").collect::<Vec<&str>>()[0].to_string();
		return (method, full_path, path_without_params);
	}
	pub fn host_line(&self) -> &str {
		&self.lines[self.host_line_index]
	}
	pub fn agent_line(&self) -> &str {
		&self.lines[self.agent_line_index]
	}
	pub fn accept_line(&self) -> &str {
		&self.lines[self.accept_line_index]
	}
}
