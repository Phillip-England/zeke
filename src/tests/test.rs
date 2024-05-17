


use chrono::format;

use crate::tests::timer::{get_time_range, log_times, Timer};
use crate::http::request::{Request, HttpMethod};

use super::timer::{Time, Times};

pub type TestResult = Result<(), String>;

#[derive(Debug, Clone)]
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

impl Copy for TestLogs {}

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

    // cleaning all our test logs
    let timer = Timer::new();
    timer.clean_log(TestLogs::HttpTest);

    wait_for_startup(host.clone()).await;  
    ping(host.clone(), 10).await; 
    invalid_method(host.clone()).await;
    missing_method(host.clone()).await;
    missing_protocol(host.clone()).await;

}

pub async fn wait_for_startup(host: String) {
    let t = Timer::new();
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
    let message = format!("wait_for_startup: {:?}", t.elapsed_message());
    t.log(TestLogs::HttpTest, &message);
}

pub async fn ping(host: String, attempts: i32) {
    for i in 0..attempts {
        let t = Timer::new();
        let req = Request::new(&host)
            .method(HttpMethod::GET)
            .path("/");
        let res = req.send();
        match res {
            Some(res) => {
                assert!(res.status == 200);
                let message = format!("ping {}: {:?}", i, t.elapsed_message());
                t.log(TestLogs::HttpTest, &message);
            },
            None => {
                assert!(false, "ping: test failed, no response")
            }
        }
    }
}

pub async fn invalid_method(host: String)  {
    let t = Timer::new();    
    let req = Request::new(&host);
    let req_malformed_method = "GE / HTTP/1.1\r\nHost: localhost\r\nConnection: close\r\n\r\n".to_string();
    let res = req.send_raw(req_malformed_method);
    match res {
        Some(res) => {
            assert!(res.status == 400);
        },
        None => {
            assert!(false, "invalid_method: test failed");
        }
    }
    t.log(TestLogs::HttpTest, &format!("invalid_method: test passed {:?}", t.elapsed_message()));
}

pub async fn missing_method(host: String) {
    let t = Timer::new();    
    let req = Request::new(&host);
    let req_missing_method = "/ HTTP/1.1\r\nHost: localhost\r\nConnection: close\r\n\r\n".to_string();
    let res = req.send_raw(req_missing_method);
    match res {
        Some(res) => {
            assert!(res.status == 400);
        },
        None => {
            assert!(false, "missing_method: test failed");
        }
    }
    t.log(TestLogs::HttpTest, &format!("missing_method: test passed {:?}", t.elapsed_message()));
}

pub async fn missing_protocol(host: String) {
    let t = Timer::new();    
    let req = Request::new(&host);
    let req_missing_protocol = "GET / \r\nHost: localhost\r\nConnection: close\r\n\r\n".to_string();
    let res = req.send_raw(req_missing_protocol);
    match res {
        Some(res) => {
            assert!(res.status == 400);
        },
        None => {
            assert!(false, "missing_protocol: test failed");
        }
    }

    t.log(TestLogs::HttpTest, &format!("missing_protocol: test passed {:?}", t.elapsed_message()));
}
