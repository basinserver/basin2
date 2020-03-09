use basin2_data::entities::*;
use std::sync::{ Arc, Weak };
use std::sync::RwLock;
use crate::world::World;

pub struct EntityPosition {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}


pub struct EntityRotation {
    pub pitch: f32,
    pub yaw: f32,
}

pub struct EntityT {
    pub entity_type: EntityType,
    pub id: u32,
    pub blocks_building: bool,
    passengers: Vec<WeakEntity>,
    vehicle: Option<WeakEntity>,
    pub forced_loading: bool,
    pub world: World,
    pub pos: EntityPosition,
    pub old_pos: EntityPosition,
    pub on_ground: bool,
    pub horizontal_collision: bool,
    pub vertical_collision: bool,
    pub fall_distance: f64,
    pub rot: EntityRotation,
    pub old_rot: EntityRotation,
}

pub type Entity = Arc<RwLock<EntityT>>;
pub type WeakEntity = Weak<RwLock<EntityT>>;