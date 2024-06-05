use crate::block::Block;
use crate::block::Transaction;

#[derive(Debug)]
pub struct Blockchain {
    pub chain: Vec<Block>,
    pub difficulty: usize,
    pub pending_transactions: Vec<Transaction>,
    pub mining_reward: u64,
}

impl Blockchain {
    pub fn new() -> Self {
        let mut blockchain = Blockchain {
            chain: Vec::new(),
            difficulty: 2,
            pending_transactions: Vec::new(),
            mining_reward: 50,
        };
        blockchain.create_genesis_block();
        blockchain
    }
    
    fn create_genesis_block(&mut self) {
        let mut genesis_block = Block::new(0, vec![], String::from("0"));
        genesis_block.hash = genesis_block.calculate_hash();
        self.chain.push(genesis_block);
    }
    
    pub fn get_latest_block(&self) -> &Block {
        self.chain.last().unwrap()
    }
    
    pub fn add_transaction(&mut self, transaction: Transaction) {
        self.pending_transactions.push(transaction);
    }
    
    pub fn mine_pending_transactions(&mut self, mining_reward_address: String) {
        let latest_block = self.get_latest_block();
        let mut block = Block::new(
            latest_block.index + 1,
            self.pending_transactions.clone(),
            latest_block.hash.clone(),
        );
        block.mine_block(self.difficulty);
        
        self.chain.push(block);
        
        self.pending_transactions = vec![Transaction {
            sender: String::from(""),
            receiver: mining_reward_address,
            amount: self.mining_reward,
        }];
    }
    
    pub fn is_chain_valid(&self) -> bool {
        for i in 1..self.chain.len() {
            let current_block = &self.chain[i];
            let previous_block = &self.chain[i - 1];
            
            if current_block.hash != current_block.calculate_hash() {
                return false;
            }
            
            if current_block.previous_hash != previous_block.hash {
                return false;
            }
        }
        true
    }
}
