use std::{collections::HashMap, fmt::Error, io::{self, ErrorKind}, str::Bytes, sync::Arc, time::Duration};

use tokio::{io::{AsyncReadExt, AsyncWriteExt}, net::{TcpListener, TcpStream}, time::timeout};

use crate::router;

use super::router::Router;



pub async fn connect(listener: TcpListener, router: Arc<Router>) {
	let (socket, _) = listener.accept().await.unwrap(); // TODO
	tokio::spawn(async move {
		let (socket, timeout_result) = read_socket(socket).await;
		match timeout_result {
			Ok(request_data) => {
				println!("Request data: {:?}", request_data);
				let route = router.get("/").unwrap();
				let response = route.lock().unwrap()();
				println!("Response: {:?}", response);
			},
			Err(e) => {
				match e.kind() {
					ErrorKind::TimedOut => {
						let (mut socket, write_result) = write_socket(socket, b"HTTP/1.1 408 Request Timeout\r\n\r\n").await;
						match write_result {
							Ok(_) => {
								let _ = socket.shutdown().await;
								return;
							},
							Err(e) => {
								eprintln!("Error writing to socket: {}", e);
								return;
							}
						}
					},
					_ => {
						eprintln!("Error reading from socket: {}", e);
					}
				}
				eprintln!("Error reading from socket: {}", e);
			},
		}

	});
}

pub async fn read_socket(mut socket: TcpStream) -> (TcpStream, Result<(usize), io::Error>) {
	let mut buffer: [u8; 1024] = [0; 1024];
	let read_timeout = timeout(Duration::from_secs(5), socket.read(&mut buffer)).await;
	match read_timeout {
		Ok(Ok(request_data)) => {
			return (socket, Ok(request_data));
		},
		// unable to read from socket
        Ok(Err(e)) => {
            return (socket, Err(e));
        },
		// read timed out
        Err(_) => {
            return (socket, Err(io::Error::new(io::ErrorKind::TimedOut, "connection timed out")));
        },
	}
}

pub async fn write_socket(mut socket: TcpStream, response: &[u8]) -> (TcpStream, Result<(), io::Error>) {
	let write_timeout = timeout(Duration::from_secs(5), socket.write_all(response)).await;
	match write_timeout {
		Ok(Ok(_)) => {
			return (socket, Ok(()));
		},
		Ok(Err(e)) => {
			return (socket, Err(e));
		},
		Err(_) => {
			return (socket, Err(io::Error::new(io::ErrorKind::TimedOut, "write timed out")));
		},
	}
}



