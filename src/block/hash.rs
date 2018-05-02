extern crate bincode;
extern crate crypto;

use super::Block;
use self::bincode::{serialize};
use self::crypto::digest::Digest;
use self::crypto::sha2::Sha256;

pub const DIFFICULTY: u8 = 2u8;

pub fn make(block: &Block) -> (u64, String) {
    // serialize the block
    let encoded: Vec<u8> = serialize(block).unwrap();
    // TODO: add a match on possible error

    // create the hash
    let mut sha = Sha256::new();
    sha.input(&encoded);
    (0, sha.result_str())
}
