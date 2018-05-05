# Rusty chain

> The beginnings of a blockchain built from scratch using Rust

I wanted to learn the fundamentals of blockchain technology and I wanted to learn Rust. This is what I'm using to scratch these 2 iches. :wink:


## Setup

Before you begin, you will need [Rust installed](https://www.rust-lang.org/en-US/install.html).

In your terminal:

    git clone https://github.com/don-smith/rusty-chain.git
    cd rusty-chain && cargo run


## Progress

* Creates two linked blocks. (`src/main.rs`)
* Verifies the hash is correct. (`src/block/mod.rs`)
* Uses a simple proof of work implementation. (`src/block/hash.rs`)
* Displays duration (in seconds) of block creation. (`src/main.rs`)


## Opportunistic documentation ;)

A `Block` is a `struct` defined in `src/block/mod.rs`. To generate the hash of the block, the struct is first serialised with [`bincode`](https://crates.io/crates/bincode) (all of its properties except `curr_hash`) into a `Vec<u8>`. Then the `Sha256` module from [`crypto`](https://crates.io/crates/rust-crypto) creates a hash of the `Vec<u8>`, which is then placed in the `curr_hash` property of the new block.
