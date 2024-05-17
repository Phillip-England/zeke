


use core::time;

use crate::tests::timer::Timer;
use crate::http::request::{Request, HttpMethod};

use super::timer::Time;


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

pub struct TestResult {
    pub log_path: TestLogs,
    pub test_name: String,
    pub time: Time,
    pub request_raw: String,
    pub response_raw: String,
}

impl TestResult {
    pub fn new(log_path: TestLogs, test_name: String, time: Time, request_raw: String, response_raw: String) -> Self {
        Self {
            log_path,
            test_name,
            time,
            request_raw,
            response_raw,
        }
    }
    pub fn log(&self) {
        let timer = Timer::new();
        timer.log(self.log_path, &format!("{}: {}{}", self.test_name, self.time.time, self.time.unit.as_str()));
        timer.log(self.log_path, &format!("\treq: {:?}", self.request_raw));
        timer.log(self.log_path, &format!("\tres: {:?}", self.response_raw));
        timer.log(self.log_path, "\n");
    }
}



pub async fn test(host: String) {

    // cleaning all our test logs
    let timer = Timer::new();
    timer.clean_log(TestLogs::HttpTest);

    startup(host.clone()).await;  
    ping(host.clone(), 3).await; 
    invalid_method(host.clone()).await;
    missing_method(host.clone()).await;
    invalid_protocol(host.clone()).await;
    missing_protocol(host.clone()).await;

}

pub async fn startup(host: String) {
    let t = Timer::new();
    let req = Request::new(&host)
        .method(HttpMethod::GET)
        .path("/");
    loop {
        let res = req.send();
        match res {
            Some(res) => {
                let result = TestResult::new(
                    TestLogs::HttpTest, 
                    "STARTUP".to_string(), 
                    t.elapsed(), 
                    req.get_request_string(), 
                    res.raw()
                );
                result.log();
                assert!(res.status == 200);
                break;
            },
            None => {

            }
        }
    }
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
                let result = TestResult::new(
                    TestLogs::HttpTest, 
                    format!("PING {}", i), 
                    t.elapsed(), 
                    req.get_request_string(), 
                    res.raw()
                );
                result.log();
                assert!(res.status == 200);
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
    let res = req.send_raw(&req_malformed_method);
    match res {
        Some(res) => {
            let result = TestResult::new(
                TestLogs::HttpTest, 
                "INVALID METHOD".to_string(), 
                t.elapsed(), 
                req_malformed_method, 
                res.raw()
            );
            result.log();
            assert!(res.status == 400);
        },
        None => {
            assert!(false, "invalid_method: test failed");
        }
    }
}

pub async fn missing_method(host: String) {
    let t = Timer::new();    
    let req = Request::new(&host);
    let req_missing_method = "/ HTTP/1.1\r\nHost: localhost\r\nConnection: close\r\n\r\n".to_string();
    let res = req.send_raw(&req_missing_method);
    match res {
        Some(res) => {
            let result = TestResult::new(
                TestLogs::HttpTest, 
                "MISSING METHOD".to_string(), 
                t.elapsed(), 
                req_missing_method, 
                res.raw()
            );
            result.log();
            assert!(res.status == 400);
        },
        None => {
            assert!(false, "missing_method: test failed");
        }
    }
}

pub async fn invalid_protocol(host: String) {
    let t = Timer::new();    
    let req = Request::new(&host);
    let req_invalid_protocol = "GET .1#####@@##@#\r\nHost: localhost\r\nConnection: close\r\n\r\n".to_string();
    let res = req.send_raw(&req_invalid_protocol);
    match res {
        Some(res) => {
            println!("{:?}", res);
            let test_result = TestResult::new(
                TestLogs::HttpTest, 
                "INVALID PROTOCOL".to_string(), 
                t.elapsed(), 
                req_invalid_protocol, 
                res.raw()
            );
            test_result.log();
            assert!(res.status == 400);


        },
        None => {
            assert!(false, "invalid_protocol: test failed");
        }
    }
}

pub async fn missing_protocol(host: String) {
    let t = Timer::new();    
    let req = Request::new(&host);
    let req_missing_protocol = "GET / \r\nHost: localhost\r\nConnection: close\r\n\r\n".to_string();
    let res = req.send_raw(&req_missing_protocol);
    match res {
        Some(res) => {
            let result = TestResult::new(
                TestLogs::HttpTest, 
                "MISSING PROTOCOL".to_string(), 
                t.elapsed(), 
                req_missing_protocol, 
                res.raw()
            );
            result.log();
            assert!(res.status == 400);
        },
        None => {
            assert!(false, "missing_protocol: test failed");
        }
    }
}


