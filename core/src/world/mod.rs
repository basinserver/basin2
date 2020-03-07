use std::sync::{Arc};
use basin2_lib::result::*;
use chashmap::CHashMap;

pub mod chunk;
pub mod block;
pub mod tile_entity;
pub mod vanilla;

use chunk::Chunk;

pub trait WorldT: Send + Sync {
    fn get_chunk(&self, x: i32, z: i32) -> Result<Chunk>;

    fn save(&self);
}

pub struct BorderSettings {
    pub center: (i32, i32),
    pub damage_per_block: f32,
    pub size: i32,
    pub safe_zone: i32,
    pub lerp_target: i32,
    pub lerp_time: i32,
    pub warning_blocks: i32,
    pub warning_time: i32,
}

pub trait LevelT: Send + Sync {
    
    fn dimensions(&self) -> &CHashMap<i32, World>;

    fn get_border_settings(&self) -> &BorderSettings;
    fn day_time(&self) -> u64;
    fn game_rules(&self) -> &CHashMap<String, String>;
    fn seed(&self) -> u64;
    fn spawn(&self) -> (i32, i32, i32);
    fn time(&self) -> u64;
}

pub type World = Arc<dyn WorldT>;
pub type Level = Arc<dyn LevelT>;
