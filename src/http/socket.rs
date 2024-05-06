use std::{ sync::{Arc, Mutex}, time::Duration};

use tokio::{io::{AsyncReadExt, AsyncWriteExt}, net::{TcpListener, TcpStream}, time::timeout};

use crate::http::router::{Middlewares, Router};
use crate::http::response::to_bytes;
use crate::http::response::new_response;
use crate::http::response::not_found;
use crate::http::response::Response;
use crate::http::request::RequestBuffer;
use crate::http::request::Request;
use crate::http::request::new_request;





pub async fn connect(listener: &TcpListener, router: Arc<Router>) {
	let (socket, _) = listener.accept().await.unwrap(); // TODO: unwrap
	tokio::spawn(async move {
        let (socket, potential_response) = handle_connection(socket, router).await;
        match potential_response {
            Some(response) => {
                let response_bytes = to_bytes(response);
                let socket = write_socket(socket, &response_bytes).await;
            },
            None => {
                return
            },
        }
    });
}

pub async fn handle_connection(socket: TcpStream, router: Arc<Router>) -> (TcpStream, Option<Response>) {
    let (socket, request_bytes, potetial_response)  = read_socket(socket).await;
    if request_bytes.len() == 0 {
        return (socket, None::<Response>); // TODO: return a response from here
    }
    let request = new_request(request_bytes);
    match request {
        Some(request) => {
            let potential_response = handle_request(router, request).await;
            if potential_response.is_some() {
                return (socket, potential_response);
            }
            // TODO: return a 500 ISE response from here
            // this means that the request was not handled by the router
            return (socket, None);
        },
        None => {
            return (socket, None);
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
                            let response = handler(request);
                            return Some(response);
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
        Ok(Err(e)) => {
            for _ in 0..5 {
                let response = new_response(500, "Internal Server Error".to_string());
                let response_bytes = to_bytes(response);
                let result = socket.write_all(&response_bytes).await;
                match result {
                    Ok(_) => {
                        return (socket, buffer, None);
                    },
                    Err(_) => {
                        continue;
                    },
                }
            }
            return (socket, buffer, None);
        },
		// read timed out
        Err(_) => {
            let response = new_response(408, "Request Timeout".to_string());
            return (socket, buffer, Some(response));
        },
	}
}

pub async fn write_socket(mut socket: TcpStream, response: &[u8]) -> (TcpStream, Option<Response>) {
	let write_timeout = timeout(Duration::from_secs(5), socket.write_all(response)).await;
    match write_timeout {
		Ok(Ok(_)) => {
			return (socket, None);
		},
        // unable to write to socket
        Ok(Err(e)) => {
            // TODO: make number of failed attempts configurable
            let response = new_response(500, "Internal Server Error".to_string());
            return (socket, Some(response));
        },
        // write timed out
		Err(_) => {
            let response = new_response(408, "Request Timeout".to_string());
            return (socket, Some(response))
		},
	}
}
