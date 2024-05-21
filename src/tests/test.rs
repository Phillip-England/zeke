


use crate::http::logger::{Logger, Logs};
use crate::http::timer::{Time, Timer};
use crate::http::request::{Request, HttpMethod};

pub async fn test(host: String, log: Logger) {
    startup(host.clone(), &log).await;  
    ping(host.clone(), 3).await; 
    get_with_headers(host.clone(), &log).await;
    get_with_params(host.clone(), &log).await;
    invalid_method(host.clone(), &log).await;
    missing_method(host.clone(), &log).await;
    invalid_protocol(host.clone(), &log).await;
    missing_protocol(host.clone(), &log).await;
    post_with_body(host.clone(), &log).await;
    put_request(host.clone(), &log).await;
    delete_request(host.clone(), &log).await;
    large_payload(host.clone(), &log).await;

}

pub async fn startup(host: String, log: &Logger) {
    let req = Request::new(&host)
        .method(HttpMethod::GET)
        .path("/");
    loop {
        let res = req.send();
		log.http(Logs::HttpTest, "startup", &req, &res);
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

pub async fn get_with_params(host: String, log: &Logger) {
    let req = Request::new(&host)
        .method(HttpMethod::GET)
        .path("/test/query_params?name=zeke&age=your_mom");
    let res = req.send();
	log.http(Logs::HttpTest, "get_with_params", &req, &res);
    assert!(res.status == 200, "get_with_params: test failed");
}

pub async fn get_with_headers(host: String, log: &Logger) {
    let req = Request::new(&host)
        .method(HttpMethod::GET)
        .path("/")
        .header("Zeke", "zeke rules")
        .header("Zekes-Mom", "so does zeke's mom");
    let res = req.send();
    let zeke = res.get_header("Zeke");
    let zekes_mom = res.get_header("Zekes-Mom");
	log.http(Logs::HttpTest, "get_with_headers", &req, &res);
    assert!(zeke == Some("zeke and his mom rule!"));
    assert!(zekes_mom == Some("so does zeke's mom"));
    assert!(res.status == 200);
}

pub async fn invalid_method(host: String, log: &Logger)  {
    let req = Request::new(&host);
    let req_malformed_method = "GE / HTTP/1.1\r\nHost: localhost\r\nConnection: close\r\n\r\n".to_string();
    let res = req.send_raw(&req_malformed_method);
	log.http(Logs::HttpTest, "invalid method", &req, &res);
	assert!(res.status == 400);
}

pub async fn missing_method(host: String, log: &Logger) {
    let req = Request::new(&host);
    let req_missing_method = "/ HTTP/1.1\r\nHost: localhost\r\nConnection: close\r\n\r\n".to_string();
    let res = req.send_raw(&req_missing_method);
	log.http(Logs::HttpTest, "missing method", &req, &res);
	assert!(res.status == 400);
}

pub async fn invalid_protocol(host: String, log: &Logger) {
    let req = Request::new(&host);
    let req_invalid_protocol = "GET .1#####@@##@#\r\nHost: localhost\r\nConnection: close\r\n\r\n".to_string();
    let res = req.send_raw(&req_invalid_protocol);
	log.http(Logs::HttpTest, "invalid_protocol", &req, &res);
	assert!(res.status == 400);
}

pub async fn missing_protocol(host: String, log: &Logger) {
    let req = Request::new(&host);
    let req_missing_protocol = "GET / \r\nHost: localhost\r\nConnection: close\r\n\r\n".to_string();
    let res = req.send_raw(&req_missing_protocol);
	log.http(Logs::HttpTest, "missing_protocol", &req, &res);
	assert!(res.status == 400);
}

pub async fn post_with_body(host: String, log: &Logger) {
    let req = Request::new(&host)
        .method(HttpMethod::POST)
        .path("/test/post_with_body")
        .body("this is a post request");
    let res = req.send();
	log.http(Logs::HttpTest, "post_with_body", &req, &res);
    assert!(res.status == 200);
}

pub async fn put_request(host: String, log: &Logger) {
    let body = r#"{"name": "Zeke Updated", "age": 26}"#;
    let req = Request::new(&host)
        .method(HttpMethod::PUT)
        .path("/test/put")
        .body(body);
    let res = req.send();
	log.http(Logs::HttpTest, "put_request", &req, &res);
    assert!(res.status == 200, "put_request: test failed");
}

pub async fn delete_request(host: String, log: &Logger) {
    let req = Request::new(&host)
        .method(HttpMethod::DELETE)
        .path("/test/delete");
    let res = req.send();
	log.http(Logs::HttpTest, "delete_request", &req, &res);
    assert!(res.status == 200, "delete_request: test failed");
}

pub async fn large_payload(host: String, log: &Logger) {
    let large_body = "a".repeat(10 * 1024 * 1024); // 10 MB payload
    let req = Request::new(&host)
        .method(HttpMethod::GET)
        .path("/")
        .body(&large_body);
    let res = req.send();
	log.http(Logs::HttpTest, "large_payload", &req, &res);
    assert!(res.status == 500, "large_payload: test failed");
}
