



use std::io::Error;
use std::sync::Mutex;
use tokio::io::AsyncWriteExt;
use tokio::net::TcpStream;
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
                let router = Arc::clone(&router);
                socket::connect(listener, router).await;
            }
        },
        Err(e) => {
            ("Error: {}", e);
        },
    }
}

pub async fn handle_connection(socket: TcpStream, router: Arc<Router>) {
    let router = Arc::clone(&router);
    tokio::spawn(async move {
        let (socket, request_bytes) = socket::read_socket(socket).await;
        if request_bytes.len() == 0 {
            return
        }
        let request = request::new_request(request_bytes);
        if request.is_none() {
            return
        }
        let request = request.unwrap(); // TODO
        let route = router.get(request.method_and_path.as_str());
        match route {
            Some(route) => {
                handle_route(socket, route, request).await;
            },
            None => {
                let response = response::not_found();
                let response_bytes = response::to_bytes(response);
                let (mut socket, failed) = socket::write_socket(socket, &response_bytes).await;
                if failed {
                    return
                }
                socket.shutdown().await.unwrap();
            },
        }
    });
}

pub async fn handle_route(socket: TcpStream, route: &Arc<Mutex<Box<dyn Fn(request::Request) -> response::Response + Send + 'static>>>, request: request::Request) {
    let response = route.lock().unwrap()(request);
    let response_bytes = response::to_bytes(response);
    let (mut socket, failed) = socket::write_socket(socket, &response_bytes).await;
    if failed {
        return
    }
    socket.shutdown().await.unwrap();
}
