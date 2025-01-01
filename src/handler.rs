use hyper::{Request};

pub type Handler = Box<dyn Fn(Request<hyper::body::Incoming>) + Send + Sync>;

pub struct DebuggableHandler;

impl std::fmt::Debug for DebuggableHandler {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<handler>")
    }
}