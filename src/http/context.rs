use std::collections::HashMap;

use crate::http::request::Request;

pub type ContextKey = str;
pub type Context = HashMap<String, String>;


pub fn set_context<K: Contextable>(request: &mut Request, key: K, value: String) {
    request.context.insert(key.to_key().to_string(), value);
}

pub fn get_context<K: Contextable>(context: &Context, key: K) -> String {
    context.get(key.to_key()).cloned().unwrap_or_default()
}

pub trait Contextable {
    fn to_key(&self) -> &'static str;
}