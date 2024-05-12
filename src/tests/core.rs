use std::future::Future;

use reqwest::Method;


pub struct HttpTestHandler {
    host: String,
    client: reqwest::Client,
    pub timer: Timer,
}

impl HttpTestHandler {
    pub fn new(host: &str) -> Self {
        Self {
            host: format!("http://{}", host),
            client: reqwest::Client::new(),
            timer: Timer::new(),
        }
    }
    pub async fn ping_loop(self) {
        let test_ping = HttpTestRequest::new(Method::GET, "/");
        loop {
            let res = self.client.request(test_ping.method.clone(), self.get_url(&test_ping.path)).send().await;
            if res.is_ok() {
                break
            } else {
                
            }
        }
    }
    pub fn get_url(&self, path: &str) -> String {
        self.host.clone() + path
    }

}

pub struct HttpTestRequest {
    path: String,
    method: reqwest::Method,
}

impl HttpTestRequest {
    pub fn new(method: reqwest::Method, path: &str) -> Self {
        Self {
            path: path.to_string(),
            method,
        }
    }
}

pub struct Timer {
    start_time: std::time::Instant,
}

impl Timer {
    pub fn new() -> Self {
        Self {
            start_time: std::time::Instant::now(),
        }
    }
    pub fn print_elapsed(&self) -> u128 {
        let elapsed_micros = self.start_time.elapsed().as_micros();
        if elapsed_micros < 1000 {
            println!("elapsed time: {} microseconds", elapsed_micros);
            return elapsed_micros;
        }
        let elapsed_millis = self.start_time.elapsed().as_millis();
        println!("elapsed time: {} milliseconds", elapsed_millis);
        return elapsed_millis;
    }
    pub fn reset(&mut self) {
        self.start_time = std::time::Instant::now();
    }
    pub async fn execute<F>(&mut self, fut: F) -> u128
    where
        F: Future<Output = ()> + Send,
    {
        self.reset();
        fut.await;
        self.print_elapsed()
    }
}