use std::time::Duration;
use std::sync::{Arc, PoisonError};

use tokio::{io::{AsyncReadExt, AsyncWriteExt}, net::{TcpListener, TcpStream}, time::timeout, sync::MutexGuard};

use crate::http::router::Router;
use crate::http::middleware::Middlewares;
use crate::http::response::{Response, ResponseBytes, PotentialResponse};
use crate::http::request::{Request, RequestBuffer};
use crate::http::handler::Handler;

use super::logger::{Logger, Logs};


pub async fn connect_socket(listener: &TcpListener, router: Arc<Router>, log: Arc<Logger>) {
	let socket_result = listener.accept().await;
    match socket_result {
        Ok(socket_result) => {
            let (socket, _addr) = socket_result;
            tokio::spawn(async move {
                let (socket, response) = handle_connection(socket, router, &log).await;
                let response_bytes: ResponseBytes = response.to_bytes();
                let (mut socket, write_result) = write_socket(socket, &response_bytes).await;
                match write_result {
					// TODO: what would cause this to fail?
					// TODO: learn more about this and how to handle it
                    Some(response) => {
						log.log(Logs::ServerError, &format!("failed to write to socket: {:?}", response));
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
					// TODO: what would even cause this to fail?
					// TODO: search up the implications of shutdown failure
                    Err(e) => {
						log.log(Logs::ServerError, &format!("failed to shutdown socket: {}", e));
                    }
                };
            });
        },
		// TODO: what would even cause this to fail?
		// TODO: search up the implications of socket connection failures
        Err(e) => {
			log.log(Logs::ServerError, &format!("failed to connect to socket: {}", e));
            return;
        },
    }

}

pub async fn handle_connection(socket: TcpStream, router: Arc<Router>, log: &Arc<Logger>) -> (TcpStream, Response) {
    let (socket, request_bytes, potetial_response) = read_socket(socket).await;
    match potetial_response {
        Some(response) => {
            return (socket, response);
        },
        None => {
            if request_bytes.len() == 0 {
                // TODO: does it matter if we get any bytes?
                return (socket, Response::new().status(200));
            }
            let (request, potential_response) = Request::new_from_bytes(request_bytes, log);
            match potential_response {
                Some(response) => {
                    return (socket, response);
                },
                None => {
                    let response: Response = handle_request(router, request, log).await;
					return (socket, response);
                },
            }
        },
    }
}

pub async fn handle_request(router: Arc<Router>, request: Request, log: &Arc<Logger>) -> Response {
    let route_handler = router.routes.get(request.method_and_path.as_str());
    match route_handler {
        Some(route_handler) => {
            let potential_route: Result<MutexGuard<(Handler, Middlewares, Middlewares)>, PoisonError<MutexGuard<(Handler, Middlewares, Middlewares)>>> = Ok(route_handler.lock().await); // TODO: need to handle this ok() better
            match potential_route {
                Ok(route_handler) => {
                    let (handler, middlewares, outerwares) = &*route_handler;
                    let (request, potential_response) = handle_middleware(request, middlewares, log).await;
                    match potential_response {
                        Some(response) => {
                            return response;
                        },
                        None => {
                            let handler = handler.func.read().await;
                            let (request, handler_response) = handler(request);
                            // TODO: clean all the white space up out of the handler_response?
                            let (_, potential_response) = handle_middleware(request, outerwares, log).await;
                            match potential_response {
                                Some(response) => {
                                    return response;
                                },
                                None => {
                                    return handler_response;
                                },
                            }
                        },
                    }
                },  
                // TODO: figure out why this would happne?
                Err(poision_error) => {
					log.log(Logs::ServerError, &format!("failed to lock route handler: {:?}", poision_error));
                    return Response::new()
                        .status(500)
                        .body("failed to lock route handler")
                },
            }
        },
        None => {
            return Response::new()
                .status(404)
                .body("route not found")
        },
    }

}

pub async fn handle_middleware(mut request: Request, middlewares: &Middlewares, log: &Arc<Logger>) -> (Request, PotentialResponse) {
    if middlewares.len() == 0 {
        return (request, None);
    };
    for middleware in middlewares {
        let middleware = middleware.func.read().await;
        let potential_response = middleware(&mut request);
        match potential_response {
            Some(response) => {
                return (request, Some(response));
            },
            None => {
                continue;
            }
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
            return (socket, buffer, Some(Response::new()
                .status(500)
                .body("no data received from client connection")
            ));
        },
        Ok(Err(e)) => {
            // Handle specific I/O errors if needed
            return (socket, buffer, Some(Response::new()
                .status(500)
                .body(&format!("failed to read from socket: {}", e))
            ));
        },
        Err(_) => {
            // Timeout
            return (socket, buffer, Some(Response::new()
                .status(408)
                .body("read timeout")
            ));
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
            return (socket, Some(Response::new()
                .status(500)
                .body(&format!("failed to write to socket: {}", e))
            ));
        },
        Err(_) => {
            // Timeout
            return (socket, Some(Response::new()
                .status(408)
                .body("write timeout")
            ));
        },
    }
}
