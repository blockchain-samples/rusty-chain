#[macro_use]
extern crate serde_derive;
extern crate time;

mod block;

use block::factory;
use block::is_valid;
use time::Instant;

fn main() {
    println!("Creating Genesis block");
    let genesis_instant = Instant::now();
    let genesis_block = factory::make_genesis();
    println!("Genesis:  {:?}", genesis_block);
    println!("Genesis creation time: {:?}", genesis_instant.elapsed());
    println!("Genesis block is valid: {}", is_valid(&genesis_block));

    println!("Creating Block #2");
    let block_instant = Instant::now();
    let block_2_data = String::from("Data for block #2");
    let block_2 = factory::make_regular(&genesis_block, block_2_data);
    println!("Block #2: {:?}", block_2);
    println!("Block #2 creation time: {:?}", block_instant.elapsed());
    println!("Block #2 is valid: {}", is_valid(&block_2));
}
