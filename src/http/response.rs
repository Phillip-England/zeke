use dashmap::DashMap;


use crate::http::cookie::Cookie;


pub type PotentialResponse = Option<Response>;
pub type ResponseBytes = Vec<u8>;
pub type ResponseHeaders = DashMap<String, String>;

#[derive(Debug, Clone)]
pub struct Response {
    pub protocol: String,
    pub status: u16,
    pub body: String,
    pub headers: ResponseHeaders,
}

impl Response {
    pub fn new() -> Self {
        let res = Self {
            protocol: "HTTP/1.1".to_string(),
            status: 200,
            body: "".to_string(),
            headers: DashMap::new(),
        };
        return res;
    }
    pub fn raw(&self) -> String {
        let mut header_string = String::new(); // Mutable string to accumulate headers
        for header in &self.headers {
			let (key, value) = header.pair();
            header_string.push_str(&format!("{}: {}\r\n", key, value));
        }
        let full_response = format!(
            "{} {}\r\n{}\r\n{}",
            self.protocol, 
            self.status,
            header_string,
            self.body
        );
        return full_response;
    }
    pub fn status(mut self, status: u16) -> Self {
        self.status = status;
        return self;
    }
    pub fn body(mut self, body: &str) -> Self {
        self.body = body.to_string();
		self.headers.insert("Content-Length".to_string(), body.len().to_string());
        return self;
    }
    pub fn new_from_bytes(response_bytes: &Vec<u8>) -> Response {
        let mut response = Response::new();
        let end = response_bytes.iter().position(|&x| x == 0).unwrap_or(response_bytes.len());
        let request_string = String::from_utf8(response_bytes[..end].to_vec());
        match request_string {
            Err(e) => {
				// TODO: set up logging
				// TODO: why would this error occur?
                return Response::new()
					.status(500)
					.body(&format!("internal server error: {:?}", e));
            }
            Ok(request_string) => {
                let lines: Vec<&str> = request_string.lines().collect();
                for i in 0..lines.len() {
                    let line = lines[i];

					// skipping empty lines
					if line.len() == 0 {
						continue;
					}

                    // protocol / status
                    if i == 0 {
                        let parts: Vec<&str> = line.split(" ").collect();
                        if parts.len() < 2 {
                            return Response::new()
								.status(400)
								.body("malformed response, more than 2 parts in status line");
                        }
                        let protocol = parts[0];
                        let status = parts[1];
                        match status.parse::<u16>() {
                            Ok(status) => {
                                response.status = status;
                            }
                            Err(_) => {
                                return Response::new()
									.status(400)
									.body("malformed response, status is not a number");
                            }
                        }
                        response.protocol = protocol.to_string();
						continue;
                    }

					// body
					// TODO: why are all requests ending with "     "
					if i == lines.len() - 1 {
						let line = line.trim();
						if line.len() == 0 {
							continue;
						}
						response.body = line.to_string();
						continue;
					}


                    // headers
					// if a header doesnt contain a colon, skip it
					// TODO: is this the best way to handle this?
					// TODO: should we return an error response instead?
					if !line.contains(":") {
						continue;
					}
					let parts: Vec<&str> = line.split(":").collect();
					if parts.len() < 2 {
						return Response::new()
							.status(400)
							.body("malformed response, more than 2 parts in header line");
					}
					let key = parts[0].trim();
					let value = parts[1].trim();
					response.headers.insert(key.to_string(), value.to_string());
                
				}
                return response;
            }
        }
    }
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut header_string = String::new(); // Mutable string to accumulate headers
        for header in &self.headers {
			let (key, value) = header.pair();
            header_string.push_str(&format!("{}: {}\r\n", key, value));
        }
        // Now create the full response with status line, headers, and body
        let full_response = format!(
            "HTTP/1.1 {}\r\n{}\r\n{}",
            self.status, 
            header_string,
            self.body
        );
        full_response.into_bytes() // Convert the full response string to bytes
    }

    pub fn set_header(mut self, key: &str, value: &str) -> Self {
        self.headers.insert(key.to_string(), value.to_string());
        return self;
    }
    pub fn get_header(&self, key: &str) -> String {
		let header = self.headers.get(key);
		if header.is_none() {
			return "".to_string();
		}
		return header.unwrap().to_string();
    }

	// Set-Cookie: sessionId=abc123; Expires=Wed, 09 Jun 2021 10:18:14 GMT; Max-Age=3600; Domain=example.com; Path=/; Secure; HttpOnly; SameSite=Strict
	pub fn set_cookie(mut self, cookie: Cookie) -> Self {
		let cookies = self.get_header("Set-Cookie");
        if cookies.len() == 0 {
            self.headers.insert("Set-Cookie".to_string(), cookie.to_string());
            return self;
        }
		let cookies_split = cookies.split(";").collect::<Vec<&str>>();
        for line in cookies_split {
            // skipping all cookies lines that dont contain the cookie name
            // if !line.contains(&cookie.name) {
            //     continue;
            // }
        }
		// for cookie in cookies {
		// 	if !cookie.contains("=") {
		// 		continue;
		// 	}
		// 	let parts = cookie.split("=").collect::<Vec<&str>>();
		// 	if parts.len() != 2 {
		// 		continue
		// 	}
		// 	let cookie_key = parts[0];
		// 	let cookie_value = parts[1];
		// 	if cookie_key == key {
		// 		continue;
		// 	}
		// 	self.headers.insert("Set-Cookie".to_string(), format!("{}={};", key, value));
		// }
		return self;
	}

	pub fn get_cookie(&self, key: &str) -> String {
		let cookies = self.headers.get("Cookie");
		if cookies.is_none() {
			return "".to_string();
		}
		let cookies = cookies.unwrap();
		let cookies = cookies.to_owned();
		let cookies = cookies.split(";").collect::<Vec<&str>>();
		for cookie in cookies {
			let parts = cookie.split("=").collect::<Vec<&str>>();
			if parts.len() != 2 {
				continue
			}
			if parts[0] == key {
				return parts[1].to_string();
			}
		}
		return "".to_string();
	}

}

pub fn not_found() -> Response {
    Response {
        protocol: "HTTP/1.1".to_string(),
        status: 404,
        body: "Not Found".to_string(),
        headers: DashMap::new(),
    }
}
