extern crate bincode;
extern crate crypto;

use super::Block;
use self::bincode::{serialize};
use self::crypto::sha2::Sha256;
use self::crypto::digest::Digest;

pub const DIFFICULTY: u8 = 2;

pub fn make(block: &Block) -> (u64, String) {
    let mut nonce: u64 = 0;
    let mut sha = Sha256::new();

    loop {
        // create a temporary block
        let temp_block = Block {
            index: block.index,
            timestamp: block.timestamp,
            data: block.data.clone(),
            nonce: Some(nonce),
            difficulty: DIFFICULTY,
            prev_hash: block.prev_hash.clone(),
            curr_hash: None
        };

        // serialize the block
        let encoded: Vec<u8> = serialize(&temp_block).unwrap();
        // TODO: add a match on possible error

        // create the hash of the block
        sha.input(&encoded);
        let hash = &mut[0; 32];
        sha.result(hash);

        // determine if the hash satisfies the proof of work
        if is_valid(hash, DIFFICULTY) {
            return (nonce, sha.result_str())
        } else {
            sha.reset();
            nonce += 1;
        }
        // println!("Nonce: {}", nonce);
    }
}

fn is_valid(hash: &[u8], difficulty: u8) -> bool {
    // A valid hash takes work to create. The work is creating a hash with a
    // specific number of leading zeros. This can only be done by repeatedly
    // creating a hash with an increasing nonce value until the hash has the
    // requisite number of leading zeros.
    let mut count: usize = difficulty as usize;
    let mut valid = true;
    while count > 0 {
        valid = valid && hash[count - 1] == 0;
        count = count - 1;
    }
    // println!("{} -> {:?}", difficulty, hash);
    valid
}
