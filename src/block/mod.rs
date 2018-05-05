mod hash;
mod utils;
pub mod factory;

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Block {
    index: u64,  // The index of the block
    timestamp: u64, // Time stamp in epoch
    data: String, // The data we want to store
    nonce: Option<u64>, // String that needs to be mined
    difficulty: u8, // Number of leading zeros in the hash
    pub prev_hash: String, // hash of previous block
    pub curr_hash: Option<String> // Current hash
}

pub fn is_valid(block: &Block) -> bool {
    let temp_block = Block {
        index: block.index,
        timestamp: block.timestamp,
        data: block.data.clone(),
        nonce: block.nonce, // not factored into current hash
        difficulty: block.difficulty,
        prev_hash: block.prev_hash.clone(),
        curr_hash: None // not factored into current hash
    };

    let (_, block_hash) = hash::make(&temp_block);
    block.curr_hash.clone().unwrap() == block_hash
}
