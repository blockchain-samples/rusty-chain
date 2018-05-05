#[macro_use]
extern crate serde_derive;
extern crate time;

mod block;

use block::factory;
use block::is_valid;
use time::PreciseTime;

fn main() {
    println!("Creating Genesis block");
    let mut start = PreciseTime::now();
    let genesis_block = factory::make_genesis();
    let mut end = PreciseTime::now();
    println!("Genesis:  {:?}", genesis_block);
    println!("Genesis creation time: {}", start.to(end));
    println!("Genesis block is valid: {}", is_valid(&genesis_block));

    println!("Creating Block #2");
    start = PreciseTime::now();
    let block_2_data = String::from("Data for block #2");
    let block_2 = factory::make_regular(&genesis_block, block_2_data);
    end = PreciseTime::now();
    println!("Block #2: {:?}", block_2);
    println!("Block #2 creation time: {}", start.to(end));
    println!("Block #2 is valid: {}", is_valid(&block_2));
}
