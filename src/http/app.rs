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

    pub fn get_handler(&self, route: &str) -> Option<Arc<Mutex<dyn Fn(Request) -> String + Send + Sync + 'static>>> {
        let router = self.router.lock().unwrap();
        let route = router.get(route);
        match route {
            Some(route) => {
                return Some(route.clone());
            }
            None => {
                return None;
            }
        }
    }
	
}