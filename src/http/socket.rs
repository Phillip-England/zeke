use std::time::Duration;
use std::sync::{Arc, Mutex, PoisonError, MutexGuard};

use tokio::{io::{AsyncReadExt, AsyncWriteExt}, net::{TcpListener, TcpStream}, time::timeout};

use crate::http::router::{Router, RouteHandler};
use crate::http::middleware::Middlewares;
use crate::http::response::{to_bytes, new_response, not_found, Response, PotentialResponse};
use crate::http::request::{Request, new_request, RequestBuffer};
use crate::http::handler::{HandlerMutex, Handler};





pub async fn connect(listener: &TcpListener, router: Arc<Router>) {
	let socket_result = listener.accept().await;
    match socket_result {
        Ok(socket_result) => {
            let (socket, _addr) = socket_result;
            tokio::spawn(async move {
                let (socket, response) = handle_connection(socket, router).await;
                let response_bytes = to_bytes(response);
                let write_result = write_socket(socket, &response_bytes).await;
                match write_result {
                    Some(response) => {
                        // TODO: set up logging when writes fail
                        println!("failed to write to socket: {:?}", response);
                    },
                    None => {
                        return;
                    },
                }
            });
        },
        Err(_) => {
            // TODO: Set up logging for when connecting to socket fails
            return; // return if socket connection fails
        },
    }

}

pub async fn handle_connection(socket: TcpStream, router: Arc<Router>) -> (TcpStream, Response) {
    let (socket, request_bytes, potetial_response) = read_socket(socket).await;
    match potetial_response {
        Some(response) => {
            return (socket, response);
        },
        None => {
            if request_bytes.len() == 0 {
                // TODO: should this be a 500?
                return (socket, new_response(500, "read 0 bytes from client connection".to_string()));
            }
            let (request, potential_response) = new_request(request_bytes);
            match potential_response {
                Some(response) => {
                    return (socket, response);
                },
                None => {
                    let potential_response: Option<Response> = handle_request(router, request).await;
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

pub async fn handle_request(router: Arc<Router>, request: Request) -> PotentialResponse {
    let route_handler: Option<&Arc<Mutex<RouteHandler>>> = router.get(request.method_and_path.as_str());
    match route_handler {
        Some(route_handler) => {
            let potential_route: Result<MutexGuard<(HandlerMutex, Middlewares)>, PoisonError<MutexGuard<(HandlerMutex, Middlewares)>>> = route_handler.lock();
            match potential_route {
                Ok(route_handler) => {
                    let (handler, middlewares) = &*route_handler;
                    let (request, potential_response) = handle_middleware(request, middlewares.to_vec());
                    match potential_response {
                        Some(response) => {
                            return Some(response);
                        },
                        None => {
                            let handler: Result<MutexGuard<Handler>, PoisonError<MutexGuard<Box<dyn Fn(Request) -> Response + Send>>>> = handler.lock();
                            match handler {
                                Ok(handler) => {
                                    let response: Response = handler(request);
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


pub fn handle_middleware(request: Request, middlewares: Middlewares) -> (Request, PotentialResponse) {
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

pub async fn read_socket(mut socket: TcpStream) -> (TcpStream, RequestBuffer, PotentialResponse) {
    let mut buffer: [u8; 1024] = [0; 1024];
    match timeout(Duration::from_secs(5), socket.read(&mut buffer)).await {
        Ok(Ok(bytes_read)) if bytes_read > 0 => {
            // TODO: trunacate() and keep only what was read?
            return (socket, buffer, None);
        },
        Ok(Ok(_)) => {
            // No data read, potentially a graceful close
            return (socket, buffer, Some(new_response(400, "No data received".to_string())));
        },
        Ok(Err(e)) => {
            // Handle specific I/O errors if needed
            return (socket, buffer, Some(new_response(500, format!("Error reading socket: {}", e))));
        },
        Err(_) => {
            // Timeout
            return (socket, buffer, Some(new_response(408, "Read timeout".to_string())));
        },
    }
}

pub async fn write_socket(mut socket: TcpStream, response_bytes: &[u8]) -> PotentialResponse {
    match timeout(Duration::from_secs(5), socket.write_all(response_bytes)).await {
        Ok(Ok(_)) => {
            return None;
        },
        Ok(Err(e)) => {
            // TODO: set up logging
            return Some(new_response(500, format!("Failed to write to socket: {}", e)));
        },
        Err(_) => {
            // Timeout
            return Some(new_response(408, "Write timeout".to_string()));
        },
    }
}
