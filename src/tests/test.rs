


use crate::http::logger::{Logger, Logs};
use crate::http::timer::{Time, Timer};
use crate::http::request::{Request, HttpMethod};

pub async fn test(host: String, log: Logger) {
    startup(host.clone(), &log).await;  
    ping(host.clone(), 3).await; 
    get_with_headers(host.clone()).await;
    get_with_params(host.clone()).await;
    invalid_method(host.clone()).await;
    missing_method(host.clone()).await;
    invalid_protocol(host.clone()).await;
    missing_protocol(host.clone()).await;
    post_with_body(host.clone()).await;
    put_request(host.clone()).await;
    delete_request(host.clone()).await;
    large_payload(host.clone()).await;

}

pub async fn startup(host: String, log: &Logger) {
    let req = Request::new(&host)
        .method(HttpMethod::GET)
        .path("/");
    loop {
        let res = req.send();
        assert!(res.status == 200);
        break;
    }
}

pub async fn ping(host: String, attempts: i32) {
    for i in 0..attempts {
        let req = Request::new(&host)
            .method(HttpMethod::GET)
            .path("/");
        let res = req.send();
        assert!(res.status == 200);
    }
}

pub async fn get_with_params(host: String) {
    let req = Request::new(&host)
        .method(HttpMethod::GET)
        .path("/test/query_params?name=zeke&age=your_mom");
    let res = req.send();
    assert!(res.status == 200, "get_with_params: test failed");
}

pub async fn get_with_headers(host: String) {
    let req = Request::new(&host)
        .method(HttpMethod::GET)
        .path("/")
        .header("Zeke", "zeke rules")
        .header("Zekes-Mom", "so does zeke's mom");
    let res = req.send();
    let zeke = res.get_header("Zeke");
    let zekes_mom = res.get_header("Zekes-Mom");
    assert!(zeke == Some("zeke and his mom rule!"));
    assert!(zekes_mom == Some("so does zeke's mom"));
    assert!(res.status == 200);
}

pub async fn invalid_method(host: String)  {
    let req = Request::new(&host);
    let req_malformed_method = "GE / HTTP/1.1\r\nHost: localhost\r\nConnection: close\r\n\r\n".to_string();
    let res = req.send_raw(&req_malformed_method);
    match res {
        Some(res) => {
            assert!(res.status == 400);
        },
        None => {
            assert!(false, "invalid_method: test failed");
        }
    }
}

pub async fn missing_method(host: String) {
    let req = Request::new(&host);
    let req_missing_method = "/ HTTP/1.1\r\nHost: localhost\r\nConnection: close\r\n\r\n".to_string();
    let res = req.send_raw(&req_missing_method);
    match res {
        Some(res) => {
            assert!(res.status == 400);
        },
        None => {
            assert!(false, "missing_method: test failed");
        }
    }
}

pub async fn invalid_protocol(host: String) {
    let req = Request::new(&host);
    let req_invalid_protocol = "GET .1#####@@##@#\r\nHost: localhost\r\nConnection: close\r\n\r\n".to_string();
    let res = req.send_raw(&req_invalid_protocol);
    match res {
        Some(res) => {
            assert!(res.status == 400);


        },
        None => {
            assert!(false, "invalid_protocol: test failed");
        }
    }
}

pub async fn missing_protocol(host: String) {
    let req = Request::new(&host);
    let req_missing_protocol = "GET / \r\nHost: localhost\r\nConnection: close\r\n\r\n".to_string();
    let res = req.send_raw(&req_missing_protocol);
    match res {
        Some(res) => {
            assert!(res.status == 400);
        },
        None => {
            assert!(false, "missing_protocol: test failed");
        }
    }
}

pub async fn post_with_body(host: String) {
    let req = Request::new(&host)
        .method(HttpMethod::POST)
        .path("/test/post_with_body")
        .body("this is a post request");
    let res = req.send();
    assert!(res.status == 200);
}

pub async fn put_request(host: String) {
    let body = r#"{"name": "Zeke Updated", "age": 26}"#;
    let req = Request::new(&host)
        .method(HttpMethod::PUT)
        .path("/test/put")
        .body(body);
    let res = req.send();
    assert!(res.status == 200, "put_request: test failed");
}

pub async fn delete_request(host: String) {
    let req = Request::new(&host)
        .method(HttpMethod::DELETE)
        .path("/test/delete");
    let res = req.send();
    assert!(res.status == 200, "delete_request: test failed");
}

pub async fn large_payload(host: String) {
    let large_body = "a".repeat(10 * 1024 * 1024); // 10 MB payload
    let req = Request::new(&host)
        .method(HttpMethod::GET)
        .path("/")
        .body(&large_body);
    let res = req.send();
    assert!(res.status == 500, "large_payload: test failed");
}
