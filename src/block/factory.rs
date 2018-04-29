use super::Block;
use super::utils;
use super::hash;

pub fn make_genesis() -> Block {
    let prev_hash = String::from(
        "0000000000000000000000000000000000000000000000000000000000000000"
    );
    let new_block = Block {
        index: 1u64,
        timestamp: utils::make_timestamp(),
        data: String::from("This is the first block in the chain"),
        nonce: None,
        difficulty: hash::DIFFICULTY,
        prev_hash: prev_hash,
        curr_hash: None
    };

    make_hashed(&new_block)
}

pub fn make_regular(prev_block: &Block, block_data: String) -> Block {
    let new_block = Block {
        index: prev_block.index + 1,
        timestamp: utils::make_timestamp(),
        data: block_data,
        nonce: None,
        difficulty: hash::DIFFICULTY,
        prev_hash: prev_block.curr_hash.clone().unwrap(),
        curr_hash: None
    };

    make_hashed(&new_block)
}

fn make_hashed(block: &Block) -> Block {
    let (nonce, hash) = hash::make(block);
    Block {
        index: block.index,
        timestamp: block.timestamp,
        data: block.data.clone(),
        nonce: Some(nonce),
        difficulty: hash::DIFFICULTY,
        prev_hash: block.prev_hash.clone(),
        curr_hash: Some(hash)
    }
}
