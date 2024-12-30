use std::net::SocketAddr;

use http_body_util::Full;
use http_body_util::{combinators::BoxBody, BodyExt};
use hyper::body::Bytes;
use hyper::header::HeaderValue;
use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper::{Method, StatusCode};
use hyper::{Request, Response};
use hyper_util::rt::TokioIo;
use tokio::net::TcpListener;
use zeke::Router;



#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // config
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let listener = TcpListener::bind(addr).await?;

    // routes
    let mut router = Router::new();

    router.add("GET", "/", |req: Request<hyper::body::Incoming>| {});

    println!("{:?}", router);

    loop {
        let (stream, _) = listener.accept().await?;
        let io = TokioIo::new(stream);
        tokio::task::spawn(async move {
            if let Err(err) = http1::Builder::new()
                .serve_connection(io, service_fn(catch_all))
                .await
            {
                eprintln!("Error serving connection: {:?}", err);
            }
        });
    }
}

// function to catch all incoming requests
async fn catch_all(
    req: Request<hyper::body::Incoming>,
) -> Result<Response<BoxBody<Bytes, hyper::Error>>, hyper::Error> {
    match req.uri().path() {
        "/" => match req.method() {
            &Method::GET => {
                let mut res = Response::new(box_response("<h1>Hello, World!</h1>"));
                res.headers_mut()
                    .insert("Content-Type", HeaderValue::from_static("text/html"));
                return Ok(res);
            }
            _ => {
                let mut invalid_method = Response::new(box_response("invalid method"));
                *invalid_method.status_mut() = StatusCode::METHOD_NOT_ALLOWED;
                return Ok(invalid_method);
            }
        },
        _ => {
            let mut not_found = Response::new(box_response("<h1>404 not found</h1>"));
            *not_found.status_mut() = StatusCode::NOT_FOUND;
            not_found
                .headers_mut()
                .insert("Content-Type", HeaderValue::from_static("text/html"));
            return Ok(not_found);
        }
    }
}

// utility function to box up our response body
fn box_response<T: Into<Bytes>>(chunk: T) -> BoxBody<Bytes, hyper::Error> {
    Full::new(chunk.into())
        .map_err(|never| match never {})
        .boxed()
}
