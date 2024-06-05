mod block;
mod blockchain;
mod network;

use block::Transaction;
use blockchain::Blockchain;
use network::P2PNode;
use std::sync::{Arc, Mutex};
use tokio::runtime::Runtime;
use tokio::time::{sleep, Duration};

async fn retry_connect(p2p_node: Arc<P2PNode>, addr: &str, retries: usize) -> Option<tokio::net::TcpStream> {
    for _ in 0..retries {
        match p2p_node.connect(addr).await {
            Ok(stream) => return Some(stream),
            Err(e) => {
                println!("Failed to connect: {}. Retrying...", e);
                sleep(Duration::from_secs(1)).await;
            }
        }
    }
    None
}

fn main() {
    let rt = Runtime::new().unwrap();

    let blockchain = Arc::new(Mutex::new(Blockchain::new()));
    let p2p_node = Arc::new(P2PNode::new(Arc::clone(&blockchain)));

    let p2p_node_clone = Arc::clone(&p2p_node);
    rt.block_on(async move {
        tokio::spawn(async move {
            p2p_node_clone.start("127.0.0.1:8080").await;
        });

        // Retry connecting to the peer
        if let Some(mut peer) = retry_connect(Arc::clone(&p2p_node), "127.0.0.1:8080", 5).await {
            p2p_node.send_message(&mut peer, "Hello, peer!").await;
            let response = p2p_node.receive_message(&mut peer).await;
            println!("Received from peer: {}", response);
        } else {
            println!("Failed to connect to peer after multiple attempts.");
        }
    });

    blockchain.lock().unwrap().add_transaction(Transaction {
        sender: String::from("Goku"),
        receiver: String::from("Vegeta"),
        amount: 10,
    });

    println!("We Mining this bitch!...");
    blockchain.lock().unwrap().mine_pending_transactions(String::from("Miner1"));

    println!("Blockchain valid: {}", blockchain.lock().unwrap().is_chain_valid());
    println!("{:?}", blockchain);
}
