use std::{collections::HashMap, fmt::Error, io::{self, ErrorKind}, str::Bytes, sync::Arc, time::Duration};

use tokio::{io::{AsyncReadExt, AsyncWriteExt}, net::{TcpListener, TcpStream}, time::timeout};

use crate::http::router::Router;
use crate::http::response::to_bytes;
use crate::http::response::get_response;
use crate::http::request::RequestBuffer;
use crate::http::request::get_request_buffer;

use super::request::usize_to_buffer;



pub async fn connect(listener: TcpListener, router: Arc<Router>) {
	let (socket, _) = listener.accept().await.unwrap(); // TODO
	tokio::spawn(async move {
		let (socket, request_bytes) = read_socket(socket).await;
        if request_bytes.len() == 0 {
            return
        }
        let route = router.get("/").unwrap();
        let response = route.lock().unwrap()();
        let response_bytes = to_bytes(response);
        let (mut socket, write_result) = write_socket(socket, &response_bytes).await;
	});
}

pub async fn read_socket(mut socket: TcpStream) -> (TcpStream, RequestBuffer) {
	let mut buffer: RequestBuffer = get_request_buffer();
	let read_timeout = timeout(Duration::from_secs(5), socket.read(&mut buffer)).await;
	match read_timeout {
		Ok(Ok(request_data)) => {
			return (socket, usize_to_buffer(request_data));
		},
		// unable to read from socket
        Ok(Err(e)) => {
            socket.shutdown().await.unwrap();
            return (socket, get_request_buffer());
        },
		// read timed out
        Err(_) => {
            let response = get_response(408, "Request Timeout".to_string());
            let response_bytes = to_bytes(response);
            let write_result = socket.write_all(&response_bytes).await;
            match write_result {
                Ok(_) => {
                    socket.shutdown().await.unwrap();
                    return (socket, get_request_buffer());
                },
                Err(e) => {
                    return (socket, get_request_buffer());
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
            let response_overwrite = get_response(408, "Request Timeout".to_string());
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



