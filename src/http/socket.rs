use std::time::Duration;
use std::sync::{Arc, PoisonError};

use tokio::{io::{AsyncReadExt, AsyncWriteExt}, net::{TcpListener, TcpStream}, time::timeout, sync::MutexGuard};

use crate::http::router::Router;
use crate::http::middleware::Middlewares;
use crate::http::response::{Response, ResponseBytes, PotentialResponse};
use crate::http::request::{Request, RequestBuffer};
use crate::http::handler::Handler;

pub async fn connect_socket(listener: &TcpListener, router: Arc<Router>) {
	let socket_result = listener.accept().await;
	if socket_result.is_err() {
		// TODO: log
		return;
	}
	let socket_result = socket_result.unwrap();
	let (socket, _addr) = socket_result;
	tokio::spawn(async move {
		let (socket, response) = handle_connection(socket, router).await;
		let response_bytes: ResponseBytes = response.to_bytes();
		let (mut socket, err_response) = write_socket(socket, &response_bytes).await;
		if err_response.is_some() {
			// TODO: log
		}
		let shutdown_result = socket.shutdown().await;
		if shutdown_result.is_err() {
			// TODO: log
		}
		return;
	});

}

pub async fn handle_connection(socket: TcpStream, router: Arc<Router>) -> (TcpStream, Response) {
    let (socket, request_bytes, potetial_response) = read_socket(socket).await;
	if potetial_response.is_some() {
		return (socket, potetial_response.unwrap());
	}
	if request_bytes.len() == 0 {
		// TODO: does it matter if we get any bytes?
		return (socket, Response::new().status(200));
	}
	let (request, potential_response) = Request::new_from_bytes(request_bytes);
	if potential_response.is_some() {
		return (socket, potential_response.unwrap());
	}
	let response: Response = handle_request(router, request).await;
	return (socket, response);
}

pub async fn handle_request(router: Arc<Router>, request: Request) -> Response {
    let route_handler = router.routes.get(request.method_and_path.as_str());
	if route_handler.is_none() {
		return Response::new()
			.status(404)
			.body("route not found");
	}
	let route_handler = route_handler.unwrap();
	let potential_route: Result<MutexGuard<(Handler, Middlewares, Middlewares)>, PoisonError<MutexGuard<(Handler, Middlewares, Middlewares)>>> = Ok(route_handler.lock().await); // TODO: need to handle this ok() better
	if potential_route.is_err() {
		return Response::new()
			.status(500)
			.body("failed to lock route handler")
	}
	let route_handler = potential_route.unwrap();
	let (handler, middlewares, outerwares) = &*route_handler;
	let (request, potential_response) = handle_middleware(request, middlewares).await;
	match potential_response {
		Some(response) => {
			return response;
		},
		None => {
			let handler = handler.func.read().await;
			let (request, handler_response) = handler(request);
			// TODO: clean all the white space up out of the handler_response?
			let (_, potential_response) = handle_middleware(request, outerwares).await;
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
}

pub async fn handle_middleware(mut request: Request, middlewares: &Middlewares) -> (Request, PotentialResponse) {
    if middlewares.len() == 0 {
        return (request, None);
    };
    for middleware in middlewares {
        let middleware = middleware.func.read().await;
        let potential_response = middleware(&mut request);
		if potential_response.is_none() {
			continue;
		}
		let response = potential_response.unwrap();
		return (request, Some(response));
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
