
use std::{
    io::{prelude::*, BufReader},
    net::TcpStream,
};

pub fn stream_to_request_vector(stream: &TcpStream) -> Vec<String> {
    let buf_reader: BufReader<&TcpStream> = BufReader::new(&stream);
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();
    http_request
}