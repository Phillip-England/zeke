
use crate::http::request::Contextable;

pub enum AppContext {
    Trace,
}

impl Contextable for AppContext {
    fn key(&self) -> &'static str {
        match self {
            AppContext::Trace => {"TRACE"},
        }
    }
}
