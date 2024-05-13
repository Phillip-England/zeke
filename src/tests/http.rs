

use tokio::task;

use crate::tests::timer::{TestLogs, Timer};
use crate::http::request::{Request, HttpMethod};




pub async fn http_test(host: String) {
    let timer = Timer::new();
    timer.clean_log(TestLogs::HttpTest);
    let (host, timer) = startup(host, timer).await;
    connect(host, timer, 10000, "10000 single connections").await;
    connect_concurrent(&host, &timer, 10, 10000, "10000 concurrent connections").await;
}

pub async fn startup(host: String, timer: Timer) -> (String, Timer) {
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
    timer = timer.print_elasped("startup");
    return (host, timer)
}

pub async fn connect(host: String, timer: Timer, number_of_connections: u32, message: &str) -> (String, Timer) {
    let req = Request::new(&host)
        .method(HttpMethod::GET)
        .path("/");
    for _ in 0..number_of_connections {
        let res = req.send();
        match res {
            Some(res) => {
                assert!(res.status == 200)
            },
            None => {

            }
        }
    }
    timer.print_elasped(message);
    return (host, timer)
}

pub async fn connect_concurrent(host: &String, timer: &Timer, num_threads: usize, connections_per_thread: u32, message: &str) {
    let tasks: Vec<_> = (0..num_threads).map(|_| {
        let host = host.clone();
        task::spawn(async move {
            let req = Request::new(&host)
                .method(HttpMethod::GET)
                .path("/");
            for _ in 0..connections_per_thread {
                let res = req.send();
                match res {
                    Some(res) => assert!(res.status == 200),
                    None => {}
                }
            }
        })
    }).collect();

    for task in tasks {
        task.await.expect("Task failed to complete");
    }

    timer.print_elasped(message);
}

