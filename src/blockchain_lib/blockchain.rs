use std::env;

use crate::blockchain_lib::block::*;
use crate::blockchain_lib::err::EmptyVecErr;

#[allow(dead_code)]
pub struct Blockchain {
    pub unconfirmed_transactions: Vec<String>,
    pub difficulty: i32,
    pub chain: Vec<Block>,
}

type ErrResult<T> = std::result::Result<T, EmptyVecErr>;

#[allow(dead_code)]
impl Blockchain {
    pub fn new() -> Self {
        Self {
            unconfirmed_transactions: vec![String::from("")],
            difficulty: 3,
            chain: vec![Block::new(0, vec!["".to_string()], "".to_string())],
        }
    }

    fn starts_with_zeros(&self, hash: &str) -> bool {
        let zeros: String = "0".repeat(self.difficulty as usize);
        let formated_regex = format!(r"^({})", &zeros[..]);
        let start_zeros = regex::Regex::new(&formated_regex).unwrap();
        start_zeros.is_match(&*(hash))
    }

    pub fn init(&mut self) {
        let mut genesis_block = &mut self.chain[0];
        genesis_block.hash = genesis_block.compute_hash();
    }

    pub fn last_block(&mut self) -> Block {
        let chain_length = self.chain.len();
        self.chain[chain_length - 1].clone()
    }

    pub fn proof_of_work(&self, block: &mut Block) -> String {
        let mut computed_hash = block.compute_hash();
        let mut string_start = self.starts_with_zeros(&computed_hash);

        while !string_start {
            block.nonce += 1;
            computed_hash = block.compute_hash();
            string_start = self.starts_with_zeros(&computed_hash);
        }
        println!("Hash: {}", computed_hash);

        computed_hash
    }

    pub fn add_block(&mut self, block: &mut Block, proof: &str) -> bool {
        let previous_hash = self.last_block().hash;

        if previous_hash != block.previous_hash {
            panic!("The previous hash of the last block isn't the same as the hash of the block with index {}!", block.index);
        };

        if !(self.is_valid_proof(block, proof)) {
            panic!("{} is not valid proof", proof);
        };
        block.hash = proof.to_string();

        self.chain.push(block.clone());
        true
    }

    pub fn is_valid_proof(&self, block: &mut Block, block_hash: &str) -> bool {
        self.starts_with_zeros(&block_hash) && block_hash == block.compute_hash()
    }

    pub fn add_transaction(&mut self, transaction: String) {
        println!(
            "Added transaction \"{}\" to unconfirmed transactions",
            transaction
        );
        self.unconfirmed_transactions.push(transaction);
    }

    fn res_mine(&mut self) -> ErrResult<Block> {
        let unconfirmed_transactions = self.unconfirmed_transactions.clone();
        let res = unconfirmed_transactions
            .get(1)
            .ok_or(EmptyVecErr)
            .and_then(|_| {
                let last_block = self.last_block();

                let mut new_block = Block::new(
                    last_block.index + 1,
                    self.unconfirmed_transactions.clone(),
                    last_block.hash,
                );

                let proof = self.proof_of_work(&mut new_block);

                self.add_block(&mut new_block, &proof);
                self.unconfirmed_transactions = vec!["".to_string()];

                Ok(new_block).map_err(|_: Block| EmptyVecErr)
            });
        res
    }

    pub fn mine(&mut self) {
        let mined_block = match self.res_mine() {
            Ok(val) => println!("Mined a block with index {}", val.index),
            Err(e) => {
                eprintln!("Error: {}", e);
                if env::consts::OS == "windows" {
                    std::process::exit(0x100);
                } else {
                    std::process::exit(0x0);
                }
            }
        };
        mined_block
    }
}
