use std::{collections::HashMap, fmt::Debug};
use dashmap::DashMap;

use crate::http::response::PotentialResponse;

pub type Context = DashMap<String, String>;
pub type Headers = DashMap<String, String>;

pub trait Contextable: Send + Sync + 'static {
    fn key(&self) -> &'static str;
} 

pub type RequestBuffer = [u8; 1024];

#[derive(Debug, Clone)]
pub struct Request {
    pub method_and_path: String,
    pub method: String,
    pub path: String,
    pub protocol: String,
    pub body: String,
    pub headers: Headers,
    pub context: Context,
}

impl Request {
    pub fn new(request_bytes: RequestBuffer) -> (Request, PotentialResponse) {
        return (Request::parse(request_bytes), None);
    }
    pub fn parse(request_bytes: RequestBuffer) -> Request {
        let mut request = Request{
            method_and_path: "".to_string(),
            method: "".to_string(),
            path: "".to_string(),
            protocol: "".to_string(),
            body: "".to_string(),
            headers: DashMap::new(),
            context: DashMap::new(),
        };
        let end = request_bytes.iter().position(|&x| x == 0).unwrap_or(request_bytes.len());
        let request_string = String::from_utf8(request_bytes[..end].to_vec());
        match request_string {
            Err(_) => {
                // TODO: failed to parse error here
                return request;
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
                            return request;
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
                return request;
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

