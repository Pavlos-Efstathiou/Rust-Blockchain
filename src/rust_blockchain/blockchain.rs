use crate::rust_blockchain::block::*;
use crate::rust_blockchain::err::{EmptyVecErr, ErrResult};

use std::env;
use std::fs;
use std::fs::File;
use std::io::prelude::*;

#[allow(dead_code)]
pub struct Blockchain {
    pub unconfirmed_transactions: Vec<String>,
    pub difficulty: u32,
    pub chain: Vec<Block>,
}

#[allow(dead_code)]
impl Blockchain {
    /// Returns a new instance of the Blockchain Struct
    fn new() -> Self {
        Self {
            unconfirmed_transactions: Vec::new(),
            difficulty: 3,
            chain: vec![Block::new(0, Vec::new(), "0000".to_string())],
        }
    }

    fn starts_with_zeros(&self, hash: &str) -> bool {
        let zeros: String = "0".repeat(self.difficulty as usize);
        hash.starts_with(&zeros)
    }

    /// Creates a new Blockchain struct and instantiates a genesis block
    pub fn init() -> Self {
        let mut blck_chain = Blockchain::new();
        let mut genesis_block = &mut blck_chain.chain[0];
        genesis_block.hash = genesis_block.compute_hash();

        blck_chain
    }

    /// Sets the difficulty to the provided value
    pub fn set_difficulty(&mut self, diff: u32) {
        self.difficulty = diff;
    }
    /// Returns a clone of the last block in a chain
    pub fn last_block(&mut self) -> Block {
        let chain_length = self.chain.len();
        self.chain[chain_length - 1].clone()
    }

    /// Proves that a block can be added to the chain
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

    /// Adds a block to the chain
    pub fn add_block(&mut self, block: &mut Block, proof: &str) {
        let previous_hash = self.last_block().hash;

        if previous_hash != block.previous_hash {
            panic!("The previous hash of the last block isn't the same as the hash of the block with index {}!", block.index);
        }

        if !(self.is_valid_proof(block, proof)) {
            panic!("{} is not valid proof", proof);
        }
        block.hash = proof.to_string();

        self.chain.push(block.clone());
    }

    fn is_valid_proof(&self, block: &mut Block, block_hash: &str) -> bool {
        self.starts_with_zeros(&block_hash) && block_hash == block.compute_hash()
    }

    fn res_mine(&mut self) -> ErrResult<Block> {
        let unconfirmed_transactions = self.unconfirmed_transactions.clone();
        let res = unconfirmed_transactions
            .get(0)
            .ok_or_else(|| EmptyVecErr.into())
            .and_then(|_| {
                let last_block = self.last_block();

                let mut new_block = Block::new(
                    last_block.index + 1,
                    self.unconfirmed_transactions.clone(),
                    last_block.hash,
                );

                let proof = self.proof_of_work(&mut new_block);

                self.add_block(&mut new_block, &proof);
                self.unconfirmed_transactions = Vec::new();

                Ok(new_block).map_err(|_: Block| EmptyVecErr.into())
            });
        res
    }

    /// Mines Blockchain
    pub fn mine(&mut self) -> u32 {
        let mined_block = match self.res_mine() {
            Ok(val) => {
                println!("Mined a block with index {}", val.index);
                val.index
            }
            Err(e) => {
                eprintln!("{}", e);
                if env::consts::OS == "windows" {
                    std::process::exit(1);
                } else {
                    std::process::exit(0);
                };
            }
        };
        mined_block
    }

    /// Writes all Blocks in a Blockchain into a separate file
    pub fn write_chain_to_file(&self) -> std::io::Result<()> {
        const BLOCKS_DIR: &str = "Blocks";

        match fs::read_dir(BLOCKS_DIR) {
            Err(_) => {
                fs::create_dir(BLOCKS_DIR)?;
                env::set_current_dir(BLOCKS_DIR)?;
            }
            Ok(_) => {
                fs::remove_dir_all(BLOCKS_DIR)?;
                fs::create_dir(BLOCKS_DIR)?;
                env::set_current_dir(BLOCKS_DIR)?;
            }
        }

        for blck in self.chain.iter() {
            let self_json = blck.get_json();
            let file_name = format!("Block {}.json", blck.index);
            let mut file = File::create(file_name)?;

            file.write_all(&(self_json.as_bytes()))?;
        }
        env::set_current_dir("..")?;
        Ok(())
    }

    /// Lists all transactions in a Blockchain
    pub fn list_transactions(&self) {
        for block in self.chain.iter() {
            for transaction in block.transactions.to_owned() {
                println!("{}", transaction);
            }
        }
    }
}

impl Default for Blockchain {
    fn default() -> Self {
        Self::init()
    }
}
