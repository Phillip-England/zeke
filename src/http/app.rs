use std::collections::HashMap;

use super::request::Request;




pub struct App {
	pub router: HashMap<String, Box<dyn Fn(Request) -> String>>,
}

impl App {
	pub fn new() -> App {
		return App {
			router: HashMap::new(),
		};
	}
	pub fn route(&mut self, path: &str, handler: Box<dyn Fn(Request) -> String>) {
		self.router.insert(path.to_string(), handler);
	}
	pub fn handle(&self, request: Request) -> String {
		let handler = self.router.get(&request.path_and_method);
		match handler {
			Some(handler) => {
				return handler(request);
			}
			None => {
				return "HTTP/1.1 404 NOT FOUND\r\n\r\nNot Found".to_string();
			}
		}
	}
}