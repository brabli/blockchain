use crate::transaction::Transaction;

#[derive(Debug)]
pub struct Block {
    pub prev_hash: String,
    pub transactions: Vec<Transaction>,
    pub nonce: u32,
    pub hash: String,
}

impl Block {
    pub fn calculate_hash(&self) -> String {
        let transactions = serde_json::to_string(&self.transactions)
            .expect("Failed to serialize block transactions!");

        let id = self.prev_hash.to_owned() + &transactions + &self.nonce.to_string();
        sha256::digest(id)
    }
}
