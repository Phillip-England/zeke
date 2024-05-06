



use std::io::Error;
use std::sync::Mutex;
use tokio::net::TcpListener;
use std::sync::Arc;

use crate::http::socket;
use crate::http::request;
use crate::http::response;
use crate::http::router::Router;



pub async fn serve(router: Arc<Router>, listener: Result<TcpListener, Error>) {
    match listener {
        Ok(ref listener) => {
            loop {
                let router: Arc<std::collections::HashMap<&str, Arc<Mutex<Box<dyn Fn(request::Request) -> response::Response + Send>>>>> = Arc::clone(&router);
                socket::connect(listener, router).await;
            }
        },
        Err(e) => {
            ("Error: {}", e);
        },
    }
}


