pub mod block;
pub mod transaction;
use stopwatch::Stopwatch;

use crate::{block::Block, transaction::Transaction};

#[allow(dead_code)]
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

    mine_block(genesis_block, Difficulty::Easy);
}

fn mine_block(mut block: Block, difficulty: Difficulty) -> Block {
    let prefix = "0".repeat(difficulty.value());

    let sw = Stopwatch::start_new();

    while !block.hash.starts_with(&prefix) {
        if block.nonce % 1000 == 0 && block.nonce != 0 {
            println!("{} hashes generated...", block.nonce);
        }

        block.nonce += 1;
        block.hash = block.calculate_hash();
    }

    println!(
        "Found a hash after {} attempts, taking {}ms.",
        block.nonce,
        sw.elapsed_ms()
    );

    block
}
