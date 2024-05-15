

use std::sync::Arc;

use tokio::task;

use crate::tests::timer::{get_time_range, log_times, Timer};
use crate::http::request::{Request, HttpMethod};

use super::timer::{Time, TimerUnit, Times};

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

    // waiting for our server to startup
    let time = wait_for_startup(host.clone()).await;  
    time.log(TestLogs::HttpTest, "startup time");

    // pinging the host 10 times
    // if we get anything but 200s, we will fail
    let times = ping(host.clone(), 10).await; 

    let (min_time, max_time) = get_time_range(&times).await;
    timer.log(TestLogs::HttpTest, "time range for 10 pings");
    timer.log(TestLogs::HttpTest, &format!("min time: {}{}", min_time.time, min_time.unit.as_str()));
    timer.log(TestLogs::HttpTest, &format!("max time: {}{}", max_time.time, max_time.unit.as_str()));

    log_times(&times, TestLogs::HttpTest);

}

pub async fn wait_for_startup(host: String) -> Time {
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
    t.elapsed()
}

pub async fn ping(host: String, attempts: i32) -> Times {
    let mut request_times: Times = vec![];
    for _ in 0..attempts {
        let t = Timer::new();
        let req = Request::new(&host)
            .method(HttpMethod::GET)
            .path("/");
        let res = req.send();
        match res {
            Some(res) => {
                assert!(res.status == 200);
            },
            None => {
                assert!(false, "ping: test failed, no response")
            }
        }
        request_times.push(t.elapsed());
    }
    request_times
}

