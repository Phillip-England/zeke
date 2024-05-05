use std::time::Duration;

use tokio::net::TcpListener;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::time::timeout;




#[tokio::main]
async fn main() {
    let listener: TcpListener = TcpListener::bind("127.0.0.1:8080").await.unwrap();
    loop {
        let (mut socket, _) = listener.accept().await.unwrap();
        tokio::spawn(async move {

            println!("Incoming connection from: {}", socket.peer_addr().unwrap());
            let mut buffer = [0; 1024];
            
            // handling connection timeouts
            let connection_timeout: Result<Result<usize, std::io::Error>, tokio::time::error::Elapsed> = timeout(Duration::from_secs(5), socket.read(&mut buffer)).await;
            match connection_timeout {
                Ok(Ok(n)) => {
                    let request_data: std::borrow::Cow<str> = String::from_utf8_lossy(&buffer[..n]);
                    println!("Received data: {}", request_data);
                },
                Ok(Err(e)) => {
                    eprintln!("Error reading from socket: {}", e);
                    let _ = socket.shutdown().await;
                    return
                },
                Err(_) => {
                    eprintln!("Connection timed out");
                    let _ = socket.shutdown().await;
                    return
                },
            }

            // handling write timeouts
            let response = "HTTP/1.1 200 OK\r\nContent-Length: 20\r\nContent-Type: text/html\r\n\r\n<h1>Hello World</h1>".as_bytes();
            let write_timeout = timeout(Duration::from_secs(5), socket.write_all(response)).await;
            match write_timeout {
                Ok(Ok(_)) => {
                    println!("Response sent");
                },
                Ok(Err(e)) => {
                    eprintln!("Error writing to socket: {}", e);
                    let _ = socket.shutdown().await;
                    return
                },
                Err(_) => {
                    eprintln!("Write timed out");
                    let _ = socket.shutdown().await;
                    return
                },
            }



        });
    }
}