



use std::io::Error;
use std::sync::Mutex;
use tokio::net::TcpListener;
use std::sync::Arc;

use crate::http::socket;
use crate::http::request;
use crate::http::response;
use crate::http::router::Router;

use super::router::RouteHandler;


pub async fn serve(router: Router, listener: Result<TcpListener, Error>) -> Option<Error> {
    let router = Arc::new(router);
    match listener {
        Ok(ref listener) => {
            loop {
                let router: Arc<std::collections::HashMap<&str, RouteHandler>> = Arc::clone(&router);
                socket::connect(listener, router).await;
            }
        },
        Err(e) => {
			return Some(e);
        },
    }
}


