use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use super::request::Request;

pub struct App {
	pub router: Arc<Mutex<HashMap<String, Arc<Mutex<dyn Fn(Request) -> String + Send + Sync + 'static>>>>>,
}

impl App {
	pub fn new() -> App {
		return App {
			router: Arc::new(Mutex::new(HashMap::new())),
		};
	}

		pub fn add_route<F>(&mut self, route: &str, callback: F)
		where
			F: Fn(Request) -> String + Send + Sync + 'static,
		{
			let mut router = self.router.lock().unwrap();
			let thread_safe_callback = Arc::new(Mutex::new(callback));
			router.insert(route.to_string(), thread_safe_callback);
		}
	
    // pub fn route(&mut self, path: &str, handler: Box<dyn Fn(Request) -> String>) {
    //     self.router.insert(path.to_string(), handler);
    // }
	// pub fn handle(&self, request: Request) -> String {
	// 	let handler = self.router.get(&request.path_and_method);
	// 	match handler {
	// 		Some(handler) => {
	// 			return handler(request);
	// 		}
	// 		None => {
	// 			return "HTTP/1.1 404 NOT FOUND\r\n\r\nNot Found".to_string();
	// 		}
	// 	}
	// }
}