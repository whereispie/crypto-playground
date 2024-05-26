mod blockchain {
    pub mod block;
}

use crate::blockchain::block::Block;

fn main() {
    let genesis_block = Block::new(0, String::from("Genesis Block"), String::from("0"));
    println!("{:#?}", genesis_block);
    println!("Hash: {}", genesis_block.hash());
}
