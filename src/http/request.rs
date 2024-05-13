use std::str;
use std::{fmt::Debug, io::{self, Read, Write}, net::TcpStream};
use dashmap::DashMap;
use io::Result;

use crate::http::response::{PotentialResponse, Response};

#[derive(Debug, Clone)]
pub enum HttpMethod {
    GET,
    POST,
    PUT,
    DELETE,
    PATCH,
}

impl HttpMethod {
    pub fn as_str(&self) -> &'static str {
        match *self {
            HttpMethod::GET => "GET",
            HttpMethod::POST => "POST",
            HttpMethod::PUT => "PUT",
            HttpMethod::DELETE => "DELETE",
            HttpMethod::PATCH => "PATCH",
        }
    }
}

pub type Context = DashMap<String, String>;
pub type Headers = DashMap<String, String>;

pub trait Contextable: Send + Sync + 'static {
    fn key(&self) -> &'static str;
} 

pub type RequestBuffer = [u8; 1024];

#[derive(Debug, Clone)]
pub struct Request {
    pub host: String,
    pub method_and_path: String,
    pub method: HttpMethod,
    pub path: String,
    pub protocol: String,
    pub body: String,
    pub headers: Headers,
    pub context: Context,
}

impl Request {
    pub fn new(host: &String) -> Self {
        let request = Self {
            host: host.to_string(),
            method_and_path: "".to_string(),
            method: HttpMethod::GET,
            path: "".to_string(),
            protocol: "".to_string(),
            body: "".to_string(),
            headers: DashMap::new(),
            context: DashMap::new(),
        };
        return request;
    }
    pub fn method(mut self, method: HttpMethod) -> Self {
        self.method = method;
        self
    }
    pub fn path(mut self, path: &str) -> Self {
        self.path = path.to_string();
        self
    }
    pub fn get_url(&self) -> String {
        self.host.clone() + &self.path
    }
    pub fn get_host(&self) -> String {
        self.host.clone()
    }
    pub fn get_request_string(&self) -> String {
        format!(
            "{} {} {}\r\nHost: {}\r\nConnection: close\r\n\r\n",
            self.method.as_str(),
            self.path,
            self.protocol,
            self.host
        )
    }
    pub fn send(&self) -> Option<Response> {
        let stream = TcpStream::connect(&self.get_host());
        match stream {
            Ok(mut stream) => {
                let request = self.get_request_string();
                match stream.write_all(request.as_bytes()) {
                    Ok(_) => {
                        let mut response_bytes = Vec::new();
                        match stream.read_to_end(&mut response_bytes) {
                            Ok(_) => {
                                let response = Response::new_from_bytes(&response_bytes);
                                match response {
                                    Some(response) => {
                                        return Some(response);
                                    },
                                    None => {
                                        return None;
                                    },
                                }
                            },
                            Err(_) => {
                                return None;
                            },
                        }

                    },
                    Err(_) => {
                        return None;
                    },
                }
            },
            Err(_) => {
                return None;
            },
        }
    }
    pub fn new_from_bytes(request_bytes: RequestBuffer) -> (Request, PotentialResponse) {
        match Request::parse_request_bytes(request_bytes) {
            (request, potential_response) => {
                return (request, potential_response);
            }
        }
    }
    pub fn parse_request_bytes(request_bytes: RequestBuffer) -> (Request, PotentialResponse) {
        let mut request = Request{
            method_and_path: "".to_string(),
            method: HttpMethod::GET,
            path: "".to_string(),
            protocol: "".to_string(),
            body: "".to_string(),
            host: "".to_string(),
            headers: DashMap::new(),
            context: DashMap::new(),
        };
        let end = request_bytes.iter().position(|&x| x == 0).unwrap_or(request_bytes.len());
        let request_string = String::from_utf8(request_bytes[..end].to_vec());
        match request_string {
            Err(_) => {
                // TODO: failed to parse error here
                return (request, Some(Response::new()
                    .status(400)
                    .body("failed to parse request")
                ));
            }
            Ok(request_string) => {
                let lines: Vec<&str> = request_string.lines().collect();
                for i in 0..lines.len() {
                    let line = lines[i];
                    // method, path, protocol
                    if i == 0 {
                        let parts = line.split(" ").collect::<Vec<&str>>();
                        if parts.len() != 3 {
                            // TODO: request did not have 3 parts: {method} {path} {protocol}
                            return (request, Some(Response::new()
                                .status(400)
                                .body("malformed request: invalid method")
                            ));
                        }
                        let method = parts[0];
                        let path = parts[1];
                        let protocol = parts[2];
                        request.method_and_path = format!("{} {}", method, path);
                        match method {
                            "GET" => {
                                request.method = HttpMethod::GET;
                            },
                            "POST" => {
                                request.method = HttpMethod::POST;
                            },
                            "PUT" => {
                                request.method = HttpMethod::PUT;
                            },
                            "DELETE" => {
                                request.method = HttpMethod::DELETE;
                            },
                            _ => {
                                return (request, Some(Response::new()
                                    .status(400)
                                    .body("malformed request: invalid method")
                                ));
                            },
                        }
                        request.path = path.to_string();
                        request.protocol = protocol.to_string();
                        continue
                    }
                    // request body
                    if i == lines.len() - 1 {
                        request.body = line.to_string();
                        continue
                    }
                    // headers
                    if line.len() == 0 { // empty line
                        continue
                    }
                    let trimmed_line = line.replace(" ", "");
                    let parts = trimmed_line.split(":").collect::<Vec<&str>>();
                    if parts.len() != 2 {
                        continue
                    }
                    let key = parts[0];
                    let value = parts[1];
                    request.headers.insert(key.to_string(), value.to_string());
    
                }
                return (request, None);
            }
        }
    }

    pub fn get_header(&self, key: &str) -> String {
        match self.headers.get(key) {
            Some(value) => {
                return value.to_string();
            },
            None => {
                return "".to_string();
            },
        }
    }

    pub fn get_context<K: Contextable>(&self, key: K) -> String {
        match self.context.get(key.key()) {
            Some(value) => {
                return value.to_string();
            },
            None => {
                return "".to_string();
            },
        }
    }

    pub fn set_context<K: Contextable>(&mut self, key: K, value: String) {
        self.context.insert(key.key().to_string(), value.to_string());
    }

}

