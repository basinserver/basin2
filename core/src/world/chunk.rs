use std::sync::{Arc, Weak};
use super::block::Block;

pub trait ChunkT: Send + Sync {
    fn get_block(&self, x: i32, y: i32, z: i32) -> Block;
    fn set_block(&self, x: i32, y: i32, z: i32, block: Block);
    fn get_x(&self) -> i32;
    fn get_z(&self) -> i32;
}

pub fn chunk_id(x: i32, z: i32) -> u64 {
    return ((x as u64) << 32) | (z as u64);
}

impl dyn ChunkT {
    pub fn get_id(&self) -> u64 {
        return chunk_id(self.get_x(), self.get_z());
    }
}

pub type Chunk = Arc<dyn ChunkT>;
pub type WeakChunk = Weak<dyn ChunkT>;
