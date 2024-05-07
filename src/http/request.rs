use std::collections::HashMap;

use crate::http::response::{new_response, Response};



pub type RequestBuffer = [u8; 1024];

#[derive(Debug, Clone)]
pub struct Request {
    pub method_and_path: String,
    pub method: String,
    pub path: String,
    pub protocol: String,
    pub body: String,
    pub headers: HashMap<String, String>,
}

pub fn new_request(buffer: RequestBuffer) -> (Request, Option<Response>) {
    let request = Request {
        method_and_path: "".to_string(),
        method: "".to_string(),
        path: "".to_string(),
        protocol: "".to_string(),
        body: "".to_string(),
        headers: HashMap::new(),
    };
    let (parsed_request, potential_response) = parse(request, buffer);
    return (parsed_request, potential_response);
}

/// TODO: Ensure that headers are parsed correctly. We need to test this.
pub fn parse(mut request: Request, buffer: RequestBuffer) -> (Request, Option<Response>) {
    let end = buffer.iter().position(|&x| x == 0).unwrap_or(buffer.len());
    let request_string = String::from_utf8(buffer[..end].to_vec());
    match request_string {
        Err(_) => {
            return (request, Some(new_response(500, "failed to parse request using from_utf8".to_string())));
        }
        Ok(request_string) => {
            let lines: Vec<&str> = request_string.lines().collect();
            for i in 0..lines.len() {
                let line = lines[i];
                // method, path, protocol
                if i == 0 {
                    let parts = line.split(" ").collect::<Vec<&str>>();
                    if parts.len() != 3 {
                        return (request, Some(new_response(500, "request did not have 3 parts: {method} {path} {protocol}".to_string())));
                    }
                    let method = parts[0];
                    let path = parts[1];
                    let protocol = parts[2];
                    request.method_and_path = format!("{} {}", method, path);
                    request.method = method.to_string();
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
                if (line.len() == 0) { // empty line
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


/// Get a header from a request.
/// Returns an empty string if header does not exist
pub fn get_header(request: Request, key: &str) -> (Request, String) {
    let mut header = "".to_string();
    match request.headers.get(key) {
        Some(value) => {
            header = value.to_string();
        },
        None => {
        },
    }
    return (request, header);
}