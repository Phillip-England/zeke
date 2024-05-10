use std::time::Duration;
use std::sync::{Arc, PoisonError};

use tokio::{io::{AsyncReadExt, AsyncWriteExt}, net::{TcpListener, TcpStream}, time::timeout, sync::{Mutex, MutexGuard}};

use crate::http::router::{Router, RouteHandler};
use crate::http::middleware::{Middlewares, MiddlewareFunc};
use crate::http::response::{to_bytes, new_response, not_found, Response, ResponseBytes, PotentialResponse};
use crate::http::request::{Request, new_request, RequestBuffer};
use crate::http::handler::Handler;





pub async fn connect_socket(listener: &TcpListener, router: Arc<Router>) {
	let socket_result = listener.accept().await;
    match socket_result {
        Ok(socket_result) => {
            let (socket, _addr) = socket_result;
            tokio::spawn(async move {
                let (socket, response) = handle_connection(socket, router).await;
                let response_bytes: ResponseBytes = to_bytes(response);
                let (mut socket, write_result) = write_socket(socket, &response_bytes).await;
                match write_result {
                    Some(response) => {
                        // TODO: set up logging when writes fail
                        println!("failed to write to socket: {:?}", response);
                    },
                    None => {
                        // proceed to shutdown
                    },
                }
                let shutdown_result = socket.shutdown().await;
                match shutdown_result {
                    Ok(_) => {
                        return;
                    },
                    Err(_e) => {
                        // TODO: set up logging when shutdown fails
                        // TODO: search up the implications of shutdown failure
                    }
                };
            });
        },
        Err(_) => {
            // TODO: Set up logging for when connecting to socket fails
            return;
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
                return (socket, new_response(500, "read 0 bytes from client connection"));
            }
            let (request, potential_response) = new_request(request_bytes);
            match potential_response {
                Some(response) => {
                    return (socket, response);
                },
                None => {
                    let potential_response: PotentialResponse = handle_request(router, request).await;
                    match potential_response {
                        Some(response) => {
                            return (socket, response);
                        },
                        None => {
                            return (socket, new_response(500, "failed to handle request"));
                        },
                    }
                },
            }
        },
    }
}

pub async fn handle_request(router: Arc<Router>, request: Request) -> PotentialResponse {
    let route_handler: Option<&Arc<Mutex<RouteHandler>>> = router.routes.get(request.method_and_path.as_str());
    match route_handler {
        Some(route_handler) => {
            let potential_route: Result<MutexGuard<(Handler, Middlewares, Middlewares)>, PoisonError<MutexGuard<(Handler, Middlewares, Middlewares)>>> = Ok(route_handler.lock().await); // TODO: need to handle this ok() better
            match potential_route {
                Ok(route_handler) => {
                    let (handler, middlewares, outerwares) = &*route_handler;
                    let (request, potential_response) = handle_middleware(request, middlewares.to_vec()).await;
                    match potential_response {
                        Some(response) => {
                            return Some(response);
                        },
                        None => {
                            let handler = handler.func.lock().await; // TODO: need to handle this ok() better
                            let (request, handler_response) = handler(request);
                            let (_, potential_response) = handle_middleware(request, outerwares.to_vec()).await;
                            match potential_response {
                                Some(response) => {
                                    return Some(response);
                                },
                                None => {
                                    return Some(handler_response);
                                },
                            }
                        },
                    }
                },  
                // PoisonError is a type of error that occurs when a Mutex is poisoned
                // TODO: set up logging for when a Mutex is poisoned
                Err(_poision_error) => {
                    return Some(new_response(500, "failed to lock route handler"));
                },
            }
        },
        None => {
            return Some(not_found());
        },
    }

}

pub async fn handle_middleware(mut request: Request, middlewares: Middlewares) -> (Request, PotentialResponse) {
    if middlewares.len() == 0 {
        return (request, None);
    };
    for middleware in middlewares {
        let middleware: Result<MutexGuard<MiddlewareFunc>, PoisonError<MutexGuard<MiddlewareFunc>>> = Ok(middleware.lock().await); // TODO: need to handle this ok() better
        match middleware {
            Ok(middleware) => {
                let potential_response = middleware(&mut request);
                match potential_response {
                    Some(response) => {
                        return (request, Some(response));
                    },
                    None => {
                        continue;
                    }
                }
            },
            // we had a posion error when trying to lock the middleware
            Err(_) => {
                // TODO: set up logging for when a middleware is poisoned
                return (request, Some(new_response(500, "failed to lock middleware")));
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
            return (socket, buffer, Some(new_response(400, "No data received")));
        },
        Ok(Err(e)) => {
            // Handle specific I/O errors if needed
            return (socket, buffer, Some(new_response(500, &format!("Error reading socket: {}", e))));
        },
        Err(_) => {
            // Timeout
            return (socket, buffer, Some(new_response(408, "Read timeout")));
        },
    }
}

pub async fn write_socket(mut socket: TcpStream, response_bytes: &[u8]) -> (TcpStream, PotentialResponse) {
    match timeout(Duration::from_secs(5), socket.write_all(response_bytes)).await {
        Ok(Ok(_)) => {
            return (socket, None);
        },
        Ok(Err(e)) => {
            // TODO: set up logging
            return (socket, Some(new_response(500, &format!("Failed to write to socket: {}", e))));
        },
        Err(_) => {
            // Timeout
            return (socket, Some(new_response(408, "Write timeout")));
        },
    }
}
