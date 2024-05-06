

pub type RequestBuffer = Vec<u8>;

pub fn get_request_buffer() -> RequestBuffer {
    [0; 1024].to_vec()
}

pub fn usize_to_buffer(data: usize) -> RequestBuffer {
    data.to_string().into_bytes()
}