use std::sync::{Arc};
use basin2_lib::result::*;
use basin2_protocol::network::DimensionType;

pub mod chunk;
pub mod block;
pub mod tile_entity;
pub mod vanilla;
pub mod level;
pub use level::*;

use crate::entity::Entity;
use chunk::Chunk;
use uuid::Uuid;

pub trait WorldT: Send + Sync {
    fn get_chunk(&self, x: i32, z: i32) -> Result<Chunk>;

    fn dimension(&self) -> DimensionType;

    fn save(&self);

    fn get_entity_by_id(&self, id: u32) -> Option<Entity>;

    fn get_entity_by_uuid(&self, uuid: Uuid) -> Option<Entity>;

    fn level(&self) -> Level;
}

pub type World = Arc<dyn WorldT>;
pub type Level = Arc<dyn level::LevelT>;
