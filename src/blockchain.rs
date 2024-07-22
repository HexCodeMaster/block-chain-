
use chrono::prelude::*;
use serde::{Serialize, Deserialize};
use sha2::{Sha256, Digest};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Transaction {
    pub sender: String,
    pub recipient: String,
    pub amount: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Block {
    pub index: u64,
    pub timestamp: i64,
    pub transactions: Vec<Transaction>,
    pub previous_hash: String,
    pub hash: String,
    pub nonce: u64,
}

pub struct Blockchain {
    pub blocks: Vec<Block>,
    pub pending_transactions: Vec<Transaction>,
    pub total_supply: f64,
    pub circulating_supply: f64,
    pub locked_supply: f64,
    pub creator_address: String,
    pub creation_time: i64,
}

impl Blockchain {
    pub fn new() -> Self {
        let total_supply = 100000_000_000.0;
        let locked_supply = total_supply * 0.30; // 30% locked
        let creator_supply = total_supply * 0.10; // 10% for creator
        let circulating_supply = total_supply - locked_supply - creator_supply; // Remaining 60%

        let creator_address = "0x".to_string();

        let mut blockchain = Blockchain {
            blocks: Vec::new(),
            pending_transactions: Vec::new(),
            total_supply,
            circulating_supply,
            locked_supply,
            creator_address: creator_address.clone(),
            creation_time: Utc::now().timestamp(),
        };

        // Initial transactions
        let initial_transactions = vec![
            Transaction {
                sender: "0".to_string(),
                recipient: creator_address.clone(),
                amount: creator_supply,
            },
            Transaction {
                sender: "0".to_string(),
                recipient: "circulating".to_string(), // for tracking circulating supply
                amount: circulating_supply,
            },
        ];

        blockchain.create_genesis_block(initial_transactions);
        blockchain
    }

    fn create_genesis_block(&mut self, initial_transactions: Vec<Transaction>) {
        let genesis_block = Block {
            index: 0,
            timestamp: Utc::now().timestamp(),
            transactions: initial_transactions,
            previous_hash: String::from("0"),
            hash: String::new(), // Will be set in `calculate_hash`
            nonce: 0,
        };
        self.blocks.push(self.calculate_hash(genesis_block));
    }

    pub fn create_transaction(&mut self, transaction: Transaction) {
        self.pending_transactions.push(transaction);
    }

    pub fn mine_pending_transactions(&mut self, reward_address: String) {
        self.release_locked_supply(); // Release locked supply before mining a new block

        let mut block = Block {
            index: self.blocks.len() as u64,
            timestamp: Utc::now().timestamp(),
            transactions: self.pending_transactions.clone(),
            previous_hash: self.blocks.last().unwrap().hash.clone(),
            hash: String::new(), // Will be set in `calculate_hash`
            nonce: 0,
        };

        // Add a reward transaction
        let reward_transaction = Transaction {
            sender: "0".to_string(),
            recipient: reward_address,
            amount: 0.01,
        };

        block.transactions.push(reward_transaction);
        self.pending_transactions.clear();
        self.blocks.push(self.calculate_hash(block));
    }

    fn calculate_hash(&self, mut block: Block) -> Block {
        loop {
            let hash = self.hash_block(&block);
            if &hash[..4] == "0000" { // Difficulty level
                block.hash = hash;
                return block;
            }
            block.nonce += 1;
        }
    }

    fn hash_block(&self, block: &Block) -> String {
        let block_string = format!(
            "{}{}{:?}{}{}",
            block.index, block.timestamp, block.transactions, block.previous_hash, block.nonce
        );
        let mut hasher = Sha256::new();
        hasher.update(block_string.as_bytes());
        let result = hasher.finalize();
        format!("{:x}", result)
    }

    // Function to release locked supply
    pub fn release_locked_supply(&mut self) {
        let current_time = Utc::now().timestamp();
        let years_passed = (current_time - self.creation_time) / (365 * 24 * 60 * 60);

        let release_amount = self.total_supply * 0.01 * years_passed as f64;

        if release_amount > self.locked_supply {
            self.circulating_supply += self.locked_supply;
            self.locked_supply = 0.0;
        } else {
            self.circulating_supply += release_amount;
            self.locked_supply -= release_amount;
        }
    }

    // Function to get the current state of the blockchain
    pub fn get_state(&self) -> String {
        format!(
            "Total Supply: {}\nCirculating Supply: {}\nLocked Supply: {}\nCreator Address: {}\n",
            self.total_supply, self.circulating_supply, self.locked_supply, self.creator_address
        )
    }
}

// Main function to test the blockchain
//fn main() {
    //let mut blockchain = Blockchain::new();

    // Create a transaction
    //blockchain.create_transaction(Transaction {
     //  sender: "Alice".to_string(),
       // recipient: "Bob".to_string(),
       // amount: 50.0,
   // });

    // Mine a block
    //blockchain.mine_pending_transactions("Miner1".to_string());

    // Print the state of the blockchain
    //println!("{}", blockchain.get_state());

    // Print the blocks in the blockchain
    //for block in &blockchain.blocks {
   //     println!("{:?}", block);
    //}
//}




