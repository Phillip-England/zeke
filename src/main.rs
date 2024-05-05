use std::time::Duration;

use tokio::net::TcpListener;
use tokio::io::{AsyncReadExt};
use tokio::time::timeout;

mod http;
use http::app::App;

#[tokio::main]
async fn main() {
    
    let mut app = App::new().await;
    app.serve().await;
    
}


// #[tokio::main]
// async fn main() {


    // let listener: TcpListener = TcpListener::bind("127.0.0.1:8080").await.unwrap();
    // loop {
    //     let (mut socket, _) = listener.accept().await.unwrap();
        // let mut socket_worker = http::socket::SocketWorker::new(socket);
        // tokio::spawn(async move {

        //     socket_worker.connect().await;
            // // handling write timeouts
            // let response = "HTTP/1.1 200 OK\r\nContent-Length: 20\r\nContent-Type: text/html\r\n\r\n<h1>Hello World</h1>".as_bytes();
            // let write_timeout = timeout(Duration::from_secs(5), socket.write_all(response)).await;
            // match write_timeout {W
            //     Ok(Ok(_)) => {
            //         println!("Response sent");
            //     },
            //     Ok(Err(e)) => {
            //         eprintln!("Error writing to socket: {}", e);
            //         match socket.write_all(b"HTTP/1.1 500 Internal Server Error\r\n\r\n").await {
            //             Ok(_) => {
            //                 let _ = socket.shutdown().await;
            //                 return
            //             },
            //             Err(_) => {
            //                 return
            //             }
            //         };
            //     },
            //     Err(_) => {
            //         eprintln!("Write timed out");
            //         match socket.write_all(b"HTTP/1.1 408 Request Timeout\r\n\r\n").await {
            //             Ok(_) => {
            //                 let _ = socket.shutdown().await;
            //                 return
            //             },
            //             Err(_) => {
            //                 return
            //             }
            //         };
            //     },
            // }



        // });
    // }
// }