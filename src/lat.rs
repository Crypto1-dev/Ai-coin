use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

// Represents a transaction
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Transaction {
    pub sender: String,
    pub recipient: String,
    pub amount: f64,
}

// Represents a block in the blockchain
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Block {
    pub index: u64,
    pub previous_hash: String,
    pub timestamp: u128,
    pub data: Vec<Transaction>,
    pub hash: String,
    pub nonce: u64,
}

impl Block {
    pub fn new(index: u64, previous_hash: String, data: Vec<Transaction>) -> Self {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis();
        let mut block = Block {
            index,
            previous_hash: previous_hash.clone(),
            timestamp,
            data,
            hash: String::new(),
            nonce: 0,
        };
        block.hash = block.calculate_hash();
        block
    }

    pub fn calculate_hash(&self) -> String {
        let value = format!(
            "{}{}{}{:?}{}",
            self.index, self.previous_hash, self.timestamp, self.data, self.nonce
        );
        format!("{:x}", md5::compute(value))
    }

    pub fn mine_block(&mut self, difficulty: usize) {
        let target = "0".repeat(difficulty);
        while !self.hash.starts_with(&target) {
            self.nonce += 1;
            self.hash = self.calculate_hash();
        }
    }
}

// Represents the blockchain
pub struct Blockchain {
    pub chain: Vec<Block>,
    pub difficulty: usize,
}

impl Blockchain {
    pub fn new(difficulty: usize) -> Self {
        let mut blockchain = Blockchain {
            chain: vec![],
            difficulty,
        };
        blockchain.chain.push(Blockchain::create_genesis_block());
        blockchain
    }

    pub fn create_genesis_block() -> Block {
        Block::new(0, "0".to_string(), vec![])
    }

    pub fn add_block(&mut self, data: Vec<Transaction>) {
        let previous_block = self.chain.last().unwrap();
        let mut new_block = Block::new(
            previous_block.index + 1,
            previous_block.hash.clone(),
            data,
        );
        new_block.mine_block(self.difficulty);
        self.chain.push(new_block);
    }
}
