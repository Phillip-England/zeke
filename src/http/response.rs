

#[derive(Debug)]
pub struct Response {
    pub status: u16,
    pub body: String,
}

pub fn get_response(status: u16, body: String) -> Response {
    Response {
        status,
        body,
    }
}

pub fn to_bytes(response: Response) -> Vec<u8> {
    format!("HTTP/1.1 {}\r\n\r\n{}", response.status, response.body).into_bytes()
}
