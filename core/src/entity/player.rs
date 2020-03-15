use super::*;
use crate::player::PlayerT;
use basin2_data::ENTITY_TYPES;

pub struct PlayerData {
    pub player: PlayerT,
}

impl EntityData for PlayerData {}


pub fn new_player_entity(world: World, player: &PlayerT) -> Result<EntityT> {
    let spawn_pos = world.level().spawn();
    let spawn_pos = EntityPosition { x: spawn_pos.0 as f64, y: spawn_pos.1 as f64, z: spawn_pos.2 as f64 };
    let spawn_rot = EntityRotation { pitch: 0.0, yaw: 0.0 };

    Ok(EntityT {
        entity_type: ENTITY_TYPES.get_str("minecraft:player").ok_or(basin_err!("no player entity type found"))?,
        id: player.entity_id,
        blocks_building: true,
        passengers: vec![],
        vehicle: None,
        forced_loading: false,
        world: player.level.dimensions().get(&0).ok_or(basin_err!("overworld not found"))?.clone(),
        motion: EntityPosition { x: 0.0, y: 0.0, z: 0.0 },
        pos: spawn_pos,
        old_pos: spawn_pos,
        rot: spawn_rot,
        old_rot: spawn_rot,
        on_ground: false,
        horizontal_collision: false,
        vertical_collision: false,
        fall_distance: 0.0,
        fire: 0,
        air: 300,
        mob_data: Some(MobData {
            health: 20.0,
            absorption_amount: 0.0,
            hurt_time: 0,
        }),
        data: Box::new(PlayerData { player: player.clone() }),
    })
}
