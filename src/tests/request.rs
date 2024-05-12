use std::io::{self, Write, Read};
use std::net::TcpStream;
use std::str;


pub enum HttpMethod {
    GET,
    POST,
    PUT,
    DELETE,
}

impl HttpMethod {
    pub fn as_str(&self) -> &'static str {
        match *self {
            HttpMethod::GET => "GET",
            HttpMethod::POST => "POST",
            HttpMethod::PUT => "PUT",
            HttpMethod::DELETE => "DELETE",
        }
    }
}

pub struct HttpRequest {
    pub method: HttpMethod,
    pub host: String,
    pub path: String,
}

impl HttpRequest {
    pub fn new(host: &String) -> Self {
        Self {
            method: HttpMethod::GET,
            host: host.to_string(),
            path: "/".to_string(),
        }
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
    pub fn send(&self) -> io::Result<String> {
        let stream = TcpStream::connect(&self.get_host());
        match stream {
            Ok(mut stream) => {
                
                let request = format!(
                    "{} {} HTTP/1.1\r\nHost: {}\r\nConnection: close\r\n\r\n",
                    self.method.as_str(),
                    self.path,
                    self.host
                );

                // Send the request
                stream.write_all(request.as_bytes())?;

                // Read the response
                let mut response = Vec::new();
                stream.read_to_end(&mut response)?;

                // Convert bytes to String
                let response = str::from_utf8(&response)
                    .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?
                    .to_string();

                Ok(response)


            },
            Err(e) => {
                println!("hit");
                Err(e)
            },
        }
    }
}
