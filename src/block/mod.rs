mod hash;
mod utils;
pub mod factory;

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Block {
    index: u64,  // The index of the block
    timestamp: u64, // Time stamp in epoch
    data: String, // The data we want to store
    // nonce: String, // String that needs to be mined
    // target: u8, // Number of leading zeros
    pub prev_hash: String, // chain/hash of previous block
    pub curr_hash: Option<String> // Current hash
}

pub fn is_valid(block: &Block) -> bool {
    let temp_block = Block {
        index: block.index,
        timestamp: block.timestamp,
        data: block.data.clone(),
        prev_hash: block.prev_hash.clone(),
        curr_hash: None // not factored into current hash
    };

    block.curr_hash.clone().unwrap() == hash::make(&temp_block)
}
