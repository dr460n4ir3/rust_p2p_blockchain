use std::time::{SystemTime, UNIX_EPOCH};
use scrypt::{scrypt, Params};
use hex::encode;

#[derive(Debug, Clone)]
pub struct Transaction {
    pub sender: String,
    pub receiver: String,
    pub amount: u64,
}

#[derive(Debug, Clone)]
pub struct Block {
    pub index: u64,
    pub timestamp: u128,
    pub transactions: Vec<Transaction>,
    pub previous_hash: String,
    pub hash: String,
    pub nonce: u64,
}

impl Block {
    pub fn new(index: u64, transactions: Vec<Transaction>, previous_hash: String) -> Self {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_millis();
        
        Block {
            index,
            timestamp,
            transactions,
            previous_hash,
            hash: String::new(),
            nonce: 0,
        }
    }
    
    pub fn calculate_hash(&self) -> String {
        let block_data = format!(
            "{}{}{:?}{}{}",
            self.index, self.timestamp, self.transactions, self.previous_hash, self.nonce
        );

        // Scrypt parameters
        let params = Params::recommended();
        let salt = b"some_salt";
        let mut hash = vec![0u8; 64];
        scrypt(block_data.as_bytes(), salt, &params, &mut hash).expect("Scrypt failed");

        encode(hash)
    }
    
    pub fn mine_block(&mut self, difficulty: usize) {
        let target = "0".repeat(difficulty);
        while !self.hash.starts_with(&target) {
            self.nonce += 1;
            self.hash = self.calculate_hash();
            println!("Mining... Nonce: {}, Hash: {}", self.nonce, self.hash); // Debug output
        }
    }
}
