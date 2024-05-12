

use std::env;

use crate::tests::timer::Timer;

use super::request::{HttpMethod, HttpRequest};


pub async fn http_test(host: String) {
    startup(host).await;
}

pub async fn startup(host: String) {
    let t = Timer::new();
    let req = HttpRequest::new(&host)
        .method(HttpMethod::GET)
        .path("/");
    loop {
        let res = req.send();
        match res {
            Ok(res) => {
                println!("Response: {:?}", res);
                t.print_elasped("startup time");

                break
            },
            Err(e) => {
                println!("ping failed: {:?}", e);
            },
        }
        // Intended continuous sending logic or other operations should go here
    }
}

