use crate::handler::DebuggableHandler;
use crate::handler::Handler;

pub struct Route<'a> {
    pub path: &'a str,
    pub method: &'a str,
    pub handler: Handler,
}

impl<'a> std::fmt::Debug for Route<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Route")
            .field("path", &self.path)
            .field("method", &self.method)
            .field("handler", &DebuggableHandler)
            .finish()
    }
}