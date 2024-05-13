

use crate::tests::timer::Timer;
use crate::http::request::{Request, HttpMethod};




pub async fn http_test(host: String) {
    startup(&host).await;
    ten_single_connections(&host);
}

pub async fn startup(host: &String) {
    let t = Timer::new();
    let req = Request::new(&host)
        .method(HttpMethod::GET)
        .path("/");
    loop {
        let res = req.send();
        match res {
            Some(res) => {
                assert!(res.status == 200);
                t.print_elasped("startup");
                break;
            },
            None => {

            }
        }
    }
}

pub fn ten_single_connections(host: &String) {
    let t = Timer::new();
    let req = Request::new(&host)
        .method(HttpMethod::GET)
        .path("/");
    for _ in 0..10 {
        let res = req.send();
        match res {
            Some(res) => {
                assert!(res.status == 200)
            },
            None => {

            }
        }
    }
    t.print_elasped("ten_single_connections");
}

