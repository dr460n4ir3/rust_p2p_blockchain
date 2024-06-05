use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use std::sync::{Arc, Mutex};
use crate::blockchain::Blockchain;

pub struct P2PNode {
    pub blockchain: Arc<Mutex<Blockchain>>,
}

impl P2PNode {
    pub fn new(blockchain: Arc<Mutex<Blockchain>>) -> Self {
        P2PNode { blockchain }
    }

    pub async fn start(&self, addr: &str) {
        let listener = TcpListener::bind(addr).await.unwrap();
        println!("Listening on: {}", addr);

        loop {
            let (mut socket, _) = listener.accept().await.unwrap();

            tokio::spawn(async move {
                let mut buffer = [0; 1024];
                loop {
                    let n = match socket.read(&mut buffer).await {
                        Ok(n) if n == 0 => return,
                        Ok(n) => n,
                        Err(_) => {
                            println!("Failed to read from socket; closing connection.");
                            return;
                        }
                    };

                    let message = String::from_utf8_lossy(&buffer[..n]);
                    println!("Received: {}", message);

                    let response = "Message received".to_string();
                    if socket.write_all(response.as_bytes()).await.is_err() {
                        println!("Failed to write to socket; closing connection.");
                        return;
                    }
                }
            });
        }
    }

    pub async fn connect(&self, addr: &str) -> Result<TcpStream, std::io::Error> {
        TcpStream::connect(addr).await
    }

    pub async fn send_message(&self, stream: &mut TcpStream, message: &str) {
        stream.write_all(message.as_bytes()).await.unwrap();
    }

    pub async fn receive_message(&self, stream: &mut TcpStream) -> String {
        let mut buffer = [0; 1024];
        let n = stream.read(&mut buffer).await.unwrap();
        String::from_utf8_lossy(&buffer[..n]).to_string()
    }
}
