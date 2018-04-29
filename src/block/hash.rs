extern crate bincode;
extern crate crypto;

use super::Block;
use self::bincode::{serialize};
use self::crypto::digest::Digest;
use self::crypto::sha2::Sha256;

pub fn make(block: &Block) -> String {
    // serialize the block
    // TODO: add a match on possible error
    let encoded: Vec<u8> = serialize(block).unwrap();
    // println!("encd: {:?}", encoded);

    // create hash
    let mut sha = Sha256::new();
    sha.input(&encoded);
    // println!("encd: {:?}", sha.result_str());
    sha.result_str()
}
