use std::sync::{Arc};
use crate::result::*;

mod chunk;
mod block;
mod tile_entity;

mod vanilla;

use chunk::Chunk;

pub trait WorldT: Send + Sync {
    fn get_chunk(&self, x: i32, z: i32) -> Result<Chunk>;

    fn save(&self);

}

pub type World = Arc<dyn WorldT>;
