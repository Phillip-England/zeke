

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
    line_method: String,
}

pub fn new_request(buffer: RequestBuffer) -> Request {
    let mut request = Request {
        buffer: buffer,
        line_method: "".to_string(),
    };
    let parsed_request = parse(request);
    println!("{:?}", parsed_request);
    return parsed_request;
}

pub fn parse(mut request: Request) -> Request {
    let request_string = String::from_utf8_lossy(&request.buffer);
    let request_parts: Vec<&str> = request_string.split(" ").collect();
    for part in &request_parts {
        println!("{}", part);
    }
    return request;
}