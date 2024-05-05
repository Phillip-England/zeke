use std::borrow::Cow;
use std::io::Error;
use std::time::Duration;

use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use tokio::time::timeout;


#[derive(Debug)]
pub struct SocketWorker {
    socket: TcpStream,
    buffer: [u8; 1024],
}

impl SocketWorker {

    pub fn new(socket: TcpStream) -> SocketWorker {
        SocketWorker {
            socket: socket,
            buffer: [0; 1024],
        }
    }

    pub async fn connect(&mut self) -> Result<Cow <str>, Error> {
        let connection_timeout = timeout(Duration::from_secs(5), self.socket.read(&mut self.buffer)).await;
        match connection_timeout {
            Ok(Ok(n)) => {
                let request_data: std::borrow::Cow<str> = String::from_utf8_lossy(&self.buffer[..n]);
                return Ok(request_data);
            },
            Ok(Err(e)) => {
                let error_message = format!("Error reading from socket: {}", e);
                return Err(Error::new(std::io::ErrorKind::Other, error_message));
            },
            Err(_) => {
                return Err(Error::new(std::io::ErrorKind::TimedOut, "Connection timed out"));
            },
        }
    }

}