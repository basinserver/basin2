use basin2_data::{ entities::*, ENTITY_TYPES };
use std::sync::{ Arc, Weak };
use std::sync::RwLock;
use crate::world::World;
use basin2_lib::Nbt;
use uuid::Uuid;
use basin2_lib::result::*;

pub mod player;

#[derive(Clone, Copy)]
pub struct EntityPosition {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl EntityPosition {
    pub fn parse_nbt(nbt: &Nbt) -> Result<EntityPosition> {
        let raw_pos = nbt.unwrap_list()?;
        if raw_pos.len() != 3 {
            return Err(basin_err!("invalid pos in entity nbt, invalid length: {}", raw_pos.len()));
        }
        Ok(EntityPosition { x: raw_pos[0].unwrap_f64()?, y: raw_pos[1].unwrap_f64()?, z: raw_pos[2].unwrap_f64()? })
    }
}

#[derive(Clone, Copy)]
pub struct EntityRotation {
    pub pitch: f32,
    pub yaw: f32,
}

impl EntityRotation {
    pub fn parse_nbt(nbt: &Nbt) -> Result<EntityRotation> {
        let raw_pos = nbt.unwrap_list()?;
        if raw_pos.len() != 2 {
            return Err(basin_err!("invalid rotation in entity nbt, invalid length: {}", raw_pos.len()));
        }
        Ok(EntityRotation { yaw: raw_pos[0].unwrap_f32()?, pitch: raw_pos[1].unwrap_f32()? })
    }
}

pub struct MobData {
    pub health: f32,
    pub absorption_amount: f32,
    pub hurt_time: i16,
    // TODO: rest of https://minecraft.gamepedia.com/Chunk_format/Mob
}


pub struct EntityT {
    pub entity_type: EntityType,
    pub id: u32,
    pub blocks_building: bool,
    passengers: Vec<Uuid>,
    vehicle: Option<Uuid>, // TODO: unimplemented
    pub forced_loading: bool,
    pub world: World,
    pub motion: EntityPosition,
    pub pos: EntityPosition,
    pub old_pos: EntityPosition,
    pub rot: EntityRotation,
    pub old_rot: EntityRotation,
    pub on_ground: bool,
    pub horizontal_collision: bool,
    pub vertical_collision: bool,
    pub fall_distance: f32,
    pub fire: i16,
    pub air: i16,
    pub mob_data: Option<MobData>,
    pub data: Box<dyn EntityData>,
}

fn parse_entity_uuid(most: i64, least: i64) -> Uuid {
    let mut raw: [u8; 16] = [0; 16];
    raw[..8].clone_from_slice(&most.to_be_bytes());
    raw[8..16].clone_from_slice(&most.to_be_bytes());
    Uuid::from_bytes(&raw).unwrap()
}

fn parse_entity_uuid_nbt(nbt: &Nbt) -> Result<Uuid> {
    Ok(parse_entity_uuid(nbt.child("UUIDMost")?.unwrap_i64()?, nbt.child("UUIDLeast")?.unwrap_i64()?))
}

impl EntityT {

    pub fn try_from(world: World, nbt: Nbt, data: Option<Box<dyn EntityData>>) -> Result<EntityT> {
        let entity_type = nbt.child("id")?.unwrap_str()?;
        let entity_type = ENTITY_TYPES.get_str(entity_type).ok_or(basin_err!("no entity type specified"))?;
        let mut passengers = vec![];
        for passenger in nbt.child("Passengers") {
            passengers.push(parse_entity_uuid_nbt(passenger)?)
        }
        let pos = EntityPosition::parse_nbt(nbt.child("Pos")?)?;
        let motion = EntityPosition::parse_nbt(nbt.child("Motion")?)?;
        let rot = EntityRotation::parse_nbt(nbt.child("Rotation")?)?;
        let mob_data = if nbt.child("Health").is_ok() {
            Some(MobData {
                health: nbt.child("Health")?.unwrap_f32()?,
                absorption_amount: nbt.child("AbsorptionAmount")?.unwrap_f32()?,
                hurt_time: nbt.child("HurtTime")?.unwrap_i16()?,
            })
        } else {
            None
        };
        let data = data.ok_or(basin_err!("no data specified for player")).or_else(|_| entity_type.data_manager.parse(&nbt))?;
        
        let entity = EntityT {
            entity_type,
            id: world.level().next_entity_id(),
            blocks_building: false,
            passengers,
            vehicle: None, // TODO
            forced_loading: false,
            world: world,
            motion,
            pos,
            old_pos: pos,
            rot,
            old_rot: rot,
            on_ground: nbt.child("OnGround")?.unwrap_i8()? == 1,
            horizontal_collision: false,
            vertical_collision: false,
            fall_distance: nbt.child("FallDistance")?.unwrap_f32()?,
            fire: nbt.child("Fire")?.unwrap_i16()?,
            air: nbt.child("Air")?.unwrap_i16()?,
            mob_data,
            data,
        };
        Ok(entity)
    }
}

pub type Entity = Arc<RwLock<EntityT>>;
pub type WeakEntity = Weak<RwLock<EntityT>>;
