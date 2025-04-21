use crate::blockchain::Block;

pub const DIFFICULTY_PREFIX: &str = "0000";

pub fn proof_of_work(mut block: Block) -> Block {
    while !block.hash.starts_with(DIFFICULTY_PREFIX) {
        block.nonce += 1;
        block.hash = block.calculate_hash();
    }
    block
}
