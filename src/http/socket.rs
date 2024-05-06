use std::{ sync::{Arc}, time::Duration};

use tokio::{io::{AsyncReadExt, AsyncWriteExt}, net::{TcpListener, TcpStream}, time::timeout};

use crate::http::router::Router;
use crate::http::middleware::Middlewares;

use crate::http::response::{to_bytes, new_response, not_found, Response};
use crate::http::request::{Request, new_request, RequestBuffer};





pub async fn connect(listener: &TcpListener, router: Arc<Router>) {
	let (socket, _) = listener.accept().await.unwrap(); // TODO: unwrap
	tokio::spawn(async move {
        let (socket, response) = handle_connection(socket, router).await;
        let response_bytes = to_bytes(response);
        write_socket(socket, &response_bytes).await;
    });
}

pub async fn handle_connection(socket: TcpStream, router: Arc<Router>) -> (TcpStream, Response) {
    let (socket, request_bytes, potetial_response) = read_socket(socket).await;
    match potetial_response {
        Some(response) => {
            return (socket, response);
        },
        None => {
            if request_bytes.len() == 0 {
                return (socket, new_response(500, "Read 0 bytes from client connection".to_string())); // TODO: return a response from here
            }
            let (request, potential_response) = new_request(request_bytes);
            match potential_response {
                Some(response) => {
                    return (socket, response);
                },
                None => {
                    let potential_response = handle_request(router, request).await;
                    match potential_response {
                        Some(response) => {
                            return (socket, response);
                        },
                        None => {
                            return (socket, new_response(500, "failed to handle request".to_string()));
                        },
                    }
                },
            }
        },
    }
}

pub async fn handle_request(router: Arc<Router>, request: Request) -> Option<Response> {
    let route = router.get(request.method_and_path.as_str());
    match route {
        Some(route) => {
            let potential_route = route.lock();
            match potential_route {
                Ok(route_handler) => {
                    let (handler, middlewares) = &*route_handler;
                    let (request, potential_response) = handle_middleware(request, middlewares.to_vec());
                    match potential_response {
                        Some(response) => {
                            return Some(response);
                        },
                        None => {
                            let handler = handler.lock();
                            match handler {
                                Ok(handler) => {
                                    let response = handler(request);
                                    return Some(response);
                                }
                                Err(_) => {
                                    return Some(new_response(500, "failed to lock handler".to_string()));
                                }
                            }
                        },
                    }
                },  
                Err(_) => {
                    None // TODO: return a response from here
                },
            }
        },
        None => {
            let response = not_found();
            return Some(response);
        },
    }

}


pub fn handle_middleware(request: Request, middlewares: Middlewares) -> (Request, Option<Response>) {
    if middlewares.len() == 0 {
        return (request, None);
    };
    for middleware in middlewares {
        let middleware = middleware.lock();
        match middleware {
            Ok(middleware) => {
                let (request, potential_response) = middleware(request);
                return (request, potential_response);
            },
            Err(_) => {
                continue
            },
        }
    }
    return (request, None);
}

pub async fn read_socket(mut socket: TcpStream) -> (TcpStream, RequestBuffer, Option<Response>) {
	let mut buffer: [u8; 1024] = [0; 1024];
	let read_timeout = timeout(Duration::from_secs(5), socket.read(&mut buffer)).await;
	match read_timeout {
		Ok(Ok(_number_of_bytes)) => {
			return (socket, buffer, None);
		},
		// unable to read from socket
        Ok(Err(_)) => {
            for _ in 0..5 {
                let result = socket.read(&mut buffer).await;
                match result {
                    Ok(_) => {
                        return (socket, buffer, None);
                    },
                    Err(_) => {
                        continue;
                    },
                }
            }
            return (socket, buffer, Some(new_response(500, "unable to read client socket".to_string())));
        },
		// read timed out
        Err(_) => {
            let response = new_response(408, "request timeout".to_string());
            return (socket, buffer, Some(response));
        },
	}
}

pub async fn write_socket(mut socket: TcpStream, response_bytes: &[u8]) {
	let write_timeout = timeout(Duration::from_secs(5), socket.write_all(response_bytes)).await;
    match write_timeout {
		Ok(Ok(_)) => {
            return;
		},
        // unable to write to socket
        Ok(Err(e)) => {
            // TODO: make number of failed attempts configurable
            // right not we are making 5 attemps to write to the socket
            let response = new_response(500, "failed to write to socket".to_string());
            let response_bytes = to_bytes(response);
            for _ in 0..5 {
                let result = socket.write_all(&response_bytes).await;
                match result {
                    Ok(_) => {
                        return;
                    },
                    Err(_) => {
                        continue;
                    },
                }
            }

        },
        // write timed out
		Err(_) => {
            let response = new_response(408, "Request Timeout".to_string());
            let response_bytes = to_bytes(response);
            for _ in 0..5 {
                let result = socket.write_all(&response_bytes).await;
                match result {
                    Ok(_) => {
                        return;
                    },
                    Err(_) => {
                        continue;
                    },
                }
            }


		},
	}
}
