


pub type PotentialResponse = Option<Response>;

#[derive(Debug)]
pub struct Response {
    pub status: u16,
    pub body: String,
    pub headers: ResponseHeaders,
}
pub type ResponseBytes = Vec<u8>;

pub type ResponseHeaders = Vec<(String, String)>;

pub fn new_response(status: u16, body: &str) -> Response {
    Response {
        status: status,
        body: body.to_string(),
        headers: vec![],
    }
}

pub fn to_bytes(response: Response) -> Vec<u8> {
    let mut header_string = String::new(); // Mutable string to accumulate headers
    for header in response.headers {
        header_string.push_str(&format!("{}: {}\r\n", header.0, header.1));
    }
    // Now create the full response with status line, headers, and body
    let full_response = format!(
        "HTTP/1.1 {}\r\n{}Content-Length: {}\r\n\r\n{}",
        response.status, 
        header_string,
        response.body.len(), // This assumes 'body' is a String or similar
        response.body
    );
    full_response.into_bytes() // Convert the full response string to bytes
}

pub fn set_header(mut response: Response, key: &str, value: &str) -> Response {
    response.headers.push((key.to_string(), value.to_string()));
    return response;
}

pub fn not_found() -> Response {
    Response {
        status: 404,
        body: "Not Found".to_string(),
        headers: vec![],
    }
}
