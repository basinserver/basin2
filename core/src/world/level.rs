use chashmap::CHashMap;
use super::World;

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
