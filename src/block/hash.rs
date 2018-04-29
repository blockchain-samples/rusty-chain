extern crate bincode;
extern crate crypto;

use super::Block;
use self::bincode::{serialize};
use self::crypto::digest::Digest;
use self::crypto::sha2::Sha256;

pub const DIFFICULTY: u8 = 2;

pub fn make(block: &Block) -> (u64, String) {
    let mut nonce: u64 = 1;

    while nonce < 500 {
        // create a new block from the values of
        // the block provided, applying the nonce
        let temp_block = Block {
            index: block.index,
            timestamp: block.timestamp,
            data: block.data.clone(),
            nonce: Some(nonce),
            difficulty: block.difficulty,
            prev_hash: block.prev_hash.clone(),
            curr_hash: None // not factored into current hash
        };
        // serializes the block
        // TODO: add a match on possible error
        let encoded: Vec<u8> = serialize(block).unwrap();
        // println!("encd: {:?}", encoded);

        // create a hash for the block
        let mut sha = Sha256::new();
        sha.input(&encoded);
        // println!("encd: {:?}", sha.result_str());

        if passes_constraint(&temp_block) {
            return (nonce, temp_block.curr_hash.unwrap())
        } else {
            nonce = nonce + 1;
        }
    }


    (nonce, sha.result_str())
}

fn passes_constraint(block: &Block) -> bool {
    let curr_hash = block.curr_hash.unwrap();
    let leading_zeros = get_leading_zeros(block.difficulty);
    &curr_hash[..block.difficulty] == &leading_zeros
}

fn get_leading_zeros(difficulty: u8) -> String {
    let zeros = String::from("");
    for id in 1..difficulty {
        zeros.push_str("0");
    }
    println!("Zeros: {}", zeros);
    zeros
}
