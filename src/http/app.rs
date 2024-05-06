



use std::io::Error;
use tokio::net::TcpListener;
use std::sync::Arc;

use crate::http::socket;
use crate::http::router::Router;



pub async fn serve(router: Router, listener: Result<TcpListener, Error>) -> Option<Error> {
    let router: Arc<Router> = Arc::new(router);
    match listener {
        Ok(ref listener) => {
            loop {
                let router: Arc<Router> = Arc::clone(&router); // TODO: is cloning the router bad?
                socket::connect(listener, router).await; 
            }
        },
        Err(e) => {
			return Some(e);
        },
    }
}