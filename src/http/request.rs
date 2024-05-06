

pub type RequestBuffer = [u8; 1024];

#[derive(Debug)]
pub struct Request {
    line_method: String,
}

pub fn new_request(buffer: RequestBuffer) -> Option<Request> {
    let request = Request {
        line_method: "".to_string(),
    };
    let (parsed_request, failed) = parse(request, buffer);
    if failed {
        return None;
    }
    return Some(parsed_request);
}

pub fn parse(mut request: Request, buffer: RequestBuffer) -> (Request, bool) {
    let request_string = String::from_utf8(buffer.to_vec());
    match request_string {
        Err(_) => {
            return (request, true);
        }
        Ok(request_string) => {
            let lines: Vec<&str> = request_string.lines().collect();
            request.line_method = lines[0].to_string();
            return (request, false);
        }
    }
}