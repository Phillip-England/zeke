use std::collections::HashMap;

use crate::http::request::Request;

pub type Context = HashMap<String, String>;

pub fn set_context<K: Contextable>(request: &mut Request, key: &K, value: String) {
    request.context.insert(key.keys().to_string(), value);
}

pub fn get_context<K: Contextable>(context: &Context, key: &K) -> String {
    context.get(key.keys()).cloned().unwrap_or_default()
}


pub trait Contextable: Send + Sync + 'static {
    fn keys(&self) -> &'static str;
} 