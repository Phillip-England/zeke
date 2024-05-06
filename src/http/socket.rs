use std::{ sync::{Arc, Mutex}, time::Duration};

use tokio::{io::{AsyncReadExt, AsyncWriteExt}, net::{TcpListener, TcpStream}, time::timeout};

use crate::http::router::Router;
use crate::http::response::to_bytes;
use crate::http::response::new_response;
use crate::http::response::not_found;
use crate::http::response::Response;
use crate::http::request::RequestBuffer;
use crate::http::request::Request;
use crate::http::request::new_request;




pub async fn connect(listener: &TcpListener, router: Arc<Router>) {
	let (socket, _) = listener.accept().await.unwrap(); // TODO
	tokio::spawn(async move {
        handle_connection(socket, router).await;

    });
}

pub async fn read_socket(mut socket: TcpStream) -> (TcpStream, RequestBuffer) {
	let mut buffer: [u8; 1024] = [0; 1024];
	let read_timeout = timeout(Duration::from_secs(5), socket.read(&mut buffer)).await;
	match read_timeout {
		Ok(Ok(bytes_read)) => {
			return (socket, buffer);
		},
		// unable to read from socket
        Ok(Err(e)) => {
            socket.shutdown().await.unwrap();
            return (socket, buffer);
        },
		// read timed out
        Err(_) => {
            let response = new_response(408, "Request Timeout".to_string());
            let response_bytes = to_bytes(response);
            let write_result = socket.write_all(&response_bytes).await;
            match write_result {
                Ok(_) => {
                    socket.shutdown().await.unwrap();
                    return (socket, buffer);
                },
                Err(e) => {
                    return (socket, buffer);
                },
            }
        },
	}
}


pub async fn write_socket(mut socket: TcpStream, response: &[u8]) -> (TcpStream, bool) {
	let write_timeout = timeout(Duration::from_secs(5), socket.write_all(response)).await;
    match write_timeout {
		Ok(Ok(_)) => {
			return (socket, false);
		},
        // unable to write to socket
		Ok(Err(e)) => {
            socket.shutdown().await.unwrap();
			return (socket, true);
		},
        // write timed out
		Err(_) => {
            let response_overwrite = new_response(408, "Request Timeout".to_string());
            let response_bytes = to_bytes(response_overwrite);
            let write_result = socket.write_all(&response_bytes).await;
            match write_result {
                Ok(_) => {
                    socket.shutdown().await.unwrap();
                    return (socket, true);
                },
                Err(e) => {
                    return (socket, true);
                },
            }
		},
	}
}


pub async fn handle_connection(socket: TcpStream, router: Arc<Router>) {
    let router = Arc::clone(&router);
    tokio::spawn(async move {
        let (socket, request_bytes) = read_socket(socket).await;
        if request_bytes.len() == 0 {
            return
        }
        let request = new_request(request_bytes);
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
                let response = not_found();
                let response_bytes = to_bytes(response);
                let (mut socket, failed) = write_socket(socket, &response_bytes).await;
                if failed {
                    return
                }
                socket.shutdown().await.unwrap();
            },
        }
    });
}

pub async fn handle_route(socket: TcpStream, route: &Arc<Mutex<Box<dyn Fn(Request) -> Response + Send + 'static>>>, request: Request) {
    let response = route.lock().unwrap()(request);
    let response_bytes = to_bytes(response);
    let (mut socket, failed) = write_socket(socket, &response_bytes).await;
    if failed {
        return
    }
    socket.shutdown().await.unwrap();
}

