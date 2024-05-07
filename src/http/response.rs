


pub type PotentialResponse = Option<Response>;

#[derive(Debug)]
pub struct Response {
    pub status: u16,
    pub body: String,
}
pub type ResponseBytes = Vec<u8>;

pub fn new_response(status: u16, body: String) -> Response {
    Response {
        status,
        body,
    }
}

pub fn to_bytes(response: Response) -> ResponseBytes {
    format!("HTTP/1.1 {}\r\n\r\n{}", response.status, response.body).into_bytes()
}

pub fn not_found() -> Response {
    Response {
        status: 404,
        body: "Not Found".to_string(),
    }
}
