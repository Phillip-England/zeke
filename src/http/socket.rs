use std::{ sync::Arc, time::Duration};

use tokio::{io::{AsyncReadExt, AsyncWriteExt}, net::{TcpListener, TcpStream}, time::timeout};

use crate::app::handle_connection;
use crate::http::router::Router;
use crate::http::response::to_bytes;
use crate::http::response::get_response;
use crate::http::request::RequestBuffer;




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
            let response = get_response(408, "Request Timeout".to_string());
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



