use serde::{Deserialize, Serialize};
use serde_json;
use sha256;

enum Difficulty {
    None,
    Easy,
    Medium,
    Hard,
    Extreme,
}

impl Difficulty {
    pub fn value(&self) -> usize {
        match self {
            Difficulty::None => 0,
            Difficulty::Easy => 2,
            Difficulty::Medium => 4,
            Difficulty::Hard => 5,
            Difficulty::Extreme => 6,
        }
    }
}

fn main() {
    let genesis_block = Block {
        prev_hash: String::from("0"),
        transactions: vec![
            Transaction {
                sender: String::from("Alice"),
                receiver: String::from("Bob"),
                amount: 1.0,
            },
            Transaction {
                sender: String::from("Bob"),
                receiver: String::from("Charlie"),
                amount: 0.5,
            },
        ],
        nonce: 0,
        hash: String::from(""),
    };

    let mined_block = mine_block(genesis_block, Difficulty::Easy);
    println!("{:#?}", mined_block);
}

#[derive(Serialize, Deserialize, Debug)]
struct Transaction {
    sender: String,
    receiver: String,
    amount: f32,
}

#[derive(Debug)]
struct Block {
    prev_hash: String,
    transactions: Vec<Transaction>,
    nonce: u32,
    hash: String,
}

fn hash(data: &str) -> String {
    sha256::digest(data)
}

fn mine_block(mut block: Block, difficulty: Difficulty) -> Block {
    let prefix = "0".repeat(difficulty.value());
    while !block.hash.starts_with(&prefix) {
        block.nonce += 1;
        block.hash = calculate_hash(&block);
    }

    block
}

fn calculate_hash(block: &Block) -> String {
    let transactions = serde_json::to_string(&block.transactions)
        .expect("Failed to serialize block transactions!");

    hash(&(block.prev_hash.to_owned() + &transactions + &block.nonce.to_string()))
}
