use std::fmt;

use chrono::prelude::*;

use sha2::Digest;

use serde::{Deserialize, Serialize};
use serde_json::Result;

/// Block struct.
/// Used in a Blockchain with other blocks
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Block {
    pub index: u32,
    pub transactions: Vec<String>,
    pub timestamp: String,
    pub previous_hash: String,
    pub hash: String,
    pub nonce: u32,
}

#[allow(dead_code)]
impl Block {
    /// Creates a new block
    pub fn new(block_index: u32, transaction_vec: Vec<String>, prev_hash: String) -> Self {
        let local_date_time: String = Local::now().format("%d-%m-%Y %H:%M:%S").to_string();
        Self {
            index: block_index,
            transactions: transaction_vec,
            timestamp: local_date_time,
            previous_hash: prev_hash,
            hash: String::new(),
            nonce: 0u32,
        }
    }

    fn get_json_result(&self) -> Result<String> {
        let json_data = serde_json::to_string(&self)?;
        Ok(json_data)
    }

    /// Return a Block struct's data in the JSON format
    pub fn get_json(&self) -> String {
        let result_json = self.get_json_result();
        result_json.as_ref().unwrap().to_string()
    }

    /// Returns the SHA256 hash of a Block
    pub fn compute_hash(&mut self) -> String {
        let json = self.get_json();
        format!("{:x}", sha2::Sha256::digest(json.as_bytes()))
    }
}

impl fmt::Display for Block {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "\nBlock {}\n--------------------------------\nIndex: {}\n\nHash: {}\n\nTimestamp: {}\n\nPrevious Hash: {}\n\nNonce: {}\n\nTransactions: {:?}\n--------------------------------\n",self.index, self.index, self.hash, self.timestamp, self.previous_hash, self.nonce, self.transactions)
    }
}
