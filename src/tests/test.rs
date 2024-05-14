

use std::sync::Arc;

use tokio::task;

use crate::tests::timer::Timer;
use crate::http::request::{Request, HttpMethod};

pub type TestResult = Result<(), String>;
pub enum TestLogs {
    HttpTest,
}

impl TestLogs {
    pub fn as_str(&self) -> &'static str {
        match *self {
            TestLogs::HttpTest => "http_test.log",
        }
    }
}

pub type TestFunc = fn(&mut TestTools);
pub struct TestTools {
    pub host: String,
    pub timer: Timer,
}

impl TestTools {
    pub fn new(host: String) -> Self {
        Self {
            host,
            timer: Timer::new(),
        }
    }
    pub fn get_host(&self) -> String {
        self.host.clone()
    }

}


pub async fn test(host: String) {
    for _ in 0..10{
        task::spawn(async move {
            test_startup().await
    });
    }
}

pub async fn test_startup(host: String) {
    let req = Request::new(&host)
        .method(HttpMethod::GET)
        .path("/");
    loop {
        let res = req.send();
        match res {
            Some(res) => {
                assert!(res.status == 200);
                break;
            },
            None => {

            }
        }
    }
    t.timer.print_elasped("GET /");
}
