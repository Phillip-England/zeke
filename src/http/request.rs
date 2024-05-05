
use std::{
    io::{prelude::*, BufReader},
    net::TcpStream,
};

#[derive(Debug)]
pub struct Request {
	pub raw: RawRequest,
}

impl Request {
	pub fn new(stream: &TcpStream) -> Request {
		let raw = RawRequest::new(stream);
		return Request { 
			raw: raw 
		}
	}
}

#[derive(Debug)]
pub struct RawRequest {
	pub lines: Vec<String>,
	method_line_index: usize,
	host_line_index: usize,
	agent_line_index: usize,
}

impl RawRequest {
	pub fn new(stream: &TcpStream) -> RawRequest {
		let lines = RawRequest::stream_to_lines(stream);
		RawRequest { 
			lines: lines,
			method_line_index: 0,
			host_line_index: 1,
			agent_line_index: 2,
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
	pub fn host_line(&self) -> &str {
		&self.lines[self.host_line_index]
	}
	pub fn agent_line(&self) -> &str {
		&self.lines[self.agent_line_index]
	}
}
