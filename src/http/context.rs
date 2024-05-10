use std::collections::HashMap;

use crate::http::request::Request;

pub type ContextKey = str;
pub type Context = HashMap<String, String>;


pub fn set_context(request: &mut Request, key: String, value: String) {
    request.context.insert(key, value);
}

pub fn get_context(context: &Context, key: String) -> String {
    let result = context.get(&key);
    match result {
        Some(str) => {
            return str.to_string();
        },
        None => {
            return "".to_string();
        }
    }
}