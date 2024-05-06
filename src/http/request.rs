

pub type RequestBuffer = Vec<u8>;

pub fn new_request_buffer() -> RequestBuffer {
    [0; 1024].to_vec()
}

pub fn usize_to_buffer(data: usize) -> RequestBuffer {
    data.to_string().into_bytes()
}

#[derive(Debug)]
pub struct Request {
    pub buffer: RequestBuffer,
}

pub fn new_request(buffer: RequestBuffer) -> Request {
    Request {
        buffer,
    }
}