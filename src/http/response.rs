


pub type PotentialResponse = Option<Response>;
pub type ResponseBytes = Vec<u8>;
pub type ResponseHeaders = Vec<(String, String)>;

#[derive(Debug, Clone)]
pub struct Response {
    pub status: u16,
    pub body: String,
    pub headers: ResponseHeaders,
}

impl Response {
    pub fn new(status: u16, body: &str) -> Response {
        Response {
            status: status,
            body: body.to_string(),
            headers: vec![],
        }
    }
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut header_string = String::new(); // Mutable string to accumulate headers
        for header in &self.headers {
            header_string.push_str(&format!("{}: {}\r\n", header.0, header.1));
        }
        // Now create the full response with status line, headers, and body
        let full_response = format!(
            "HTTP/1.1 {}\r\n{}Content-Length: {}\r\n\r\n{}",
            self.status, 
            header_string,
            self.body.len(), // This assumes 'body' is a String or similar
            self.body
        );
        full_response.into_bytes() // Convert the full response string to bytes
    }

    pub fn set_header(mut self, key: &str, value: &str) -> Self {
        self.headers.push((key.to_string(), value.to_string()));
        return self;
    }

}

pub fn not_found() -> Response {
    Response {
        status: 404,
        body: "Not Found".to_string(),
        headers: vec![],
    }
}
