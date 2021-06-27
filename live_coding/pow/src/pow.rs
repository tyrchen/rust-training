use crate::pb::{Block, BlockHash};
use rayon::prelude::*;
pub const PREFIX_ZERO: &[u8] = &[0, 0, 0];

#[allow(dead_code)]
pub fn pow_v1(block: Block) -> Option<BlockHash> {
    let hasher = blake3_base_hash(&block.data);
    let nonce = (0..u32::MAX).find(|n| {
        let hash = blake3_hash(hasher.clone(), *n);
        &hash[..PREFIX_ZERO.len()] == PREFIX_ZERO
    });
    nonce.map(|n| {
        let id = get_block_id(&block);
        let hash = blake3_hash(hasher, n);
        BlockHash { id, hash, nonce: n }
    })
}

pub fn pow_v2(block: Block) -> Option<BlockHash> {
    let hasher = blake3_base_hash(&block.data);
    let nonce = (0..u32::MAX).into_par_iter().find_any(|n| {
        let hash = blake3_hash(hasher.clone(), *n);
        &hash[..PREFIX_ZERO.len()] == PREFIX_ZERO
    });
    nonce.map(|n| {
        let id = get_block_id(&block);
        let hash = blake3_hash(hasher, n);
        BlockHash { id, hash, nonce: n }
    })
}

fn get_block_id(block: &Block) -> Vec<u8> {
    let hash = blake3::hash(&block.data);
    hash.as_bytes().to_vec()
}

// pow hash: Block data + nonce (BE) => hash

fn blake3_hash(mut hasher: blake3::Hasher, nonce: u32) -> Vec<u8> {
    hasher.update(&nonce.to_be_bytes()[..]);
    hasher.finalize().as_bytes().to_vec()
}

fn blake3_base_hash(data: &[u8]) -> blake3::Hasher {
    let mut hasher = blake3::Hasher::new();
    hasher.update(data);
    hasher
}
