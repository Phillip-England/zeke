



pub struct App {
    listener: tokio::net::TcpListener,
}

impl App {
    
    pub async fn new() -> App {
        let listener = tokio::net::TcpListener::bind("127.0.0.1:8080");
        match listener.await {
            Ok(listener) => {
                return App {
                    listener: listener,
                }
            },
            Err(e) => {
                panic!("Error binding to address: {}", e);
            },
        
        }
    }

    pub async fn serve(&mut self) {
        println!("Serving");
    }

}

