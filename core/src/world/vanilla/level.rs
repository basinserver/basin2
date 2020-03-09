use crate::world::level::{ LevelT, BorderSettings };
use crate::world::{ World, WorldT };
use super::VanillaWorld;
use chashmap::CHashMap;
use std::sync::atomic::{ AtomicU64, Ordering };
use basin2_lib::nbt::*;
use basin2_lib::result::*;
use std::path::{ Path, PathBuf };
use std::fs;
use bytes::BytesMut;
use std::sync::Arc;
use flate2::write::{GzDecoder};
use std::io::Write;
use basin2_protocol::network::{ DimensionType };

pub struct VanillaLevel {
    directory: PathBuf,
    dimensions: CHashMap<i32, World>,
    border_settings: BorderSettings,
    day_time: AtomicU64,
    game_rules: CHashMap<String, String>,
    seed: u64,
    spawn: (i32, i32, i32),
    time: AtomicU64,
}

impl VanillaLevel {
    pub fn new<T: AsRef<Path>>(directory: T) -> Result<VanillaLevel> {
        let directory = directory.as_ref();
        let level_file = directory.join("level.dat");
        let dimensions = CHashMap::new();
        let raw_nbt = BytesMut::from(&fs::read(level_file)?[..]);
        let mut deflater = GzDecoder::new(vec![]);
        deflater.write_all(&raw_nbt)?;
        let mut raw_nbt = BytesMut::from(&deflater.finish()?[..]);

        let level_nbt = Nbt::parse(&mut raw_nbt)?;
        let level_nbt = level_nbt.child("Data")?;
        let border_settings = {
            let center_x = level_nbt.child("BorderCenterX")?.unwrap_f64()?;
            let center_z = level_nbt.child("BorderCenterZ")?.unwrap_f64()?;
            let damage_per_block = level_nbt.child("BorderDamagePerBlock")?.unwrap_f64()?;
            let size = level_nbt.child("BorderSize")?.unwrap_f64()?;
            let safe_zone = level_nbt.child("BorderSafeZone")?.unwrap_f64()?;
            let lerp_target = level_nbt.child("BorderSizeLerpTarget")?.unwrap_f64()?;
            let lerp_time = level_nbt.child("BorderSizeLerpTime")?.unwrap_i64()?;
            let warning_blocks = level_nbt.child("BorderWarningBlocks")?.unwrap_f64()?;
            let warning_time = level_nbt.child("BorderWarningTime")?.unwrap_f64()?;
            BorderSettings {
                center: (center_x as i32, center_z as i32),
                damage_per_block: damage_per_block as f32,
                size: size as i32,
                safe_zone: safe_zone as i32,
                lerp_target: lerp_target as i32,
                lerp_time: lerp_time as i32,
                warning_blocks: warning_blocks as i32,
                warning_time: warning_time as i32,
            }
        };
        let day_time = AtomicU64::new(level_nbt.child("DayTime")?.unwrap_i64()? as u64);
        let game_rules = CHashMap::new();
        for (key, child) in level_nbt.child("GameRules")?.unwrap_compound()? {
            game_rules.insert(key.clone(), child.unwrap_str()?.to_string());
        };
        let seed = level_nbt.child("RandomSeed")?.unwrap_i64()? as u64;
        let spawn = (
            level_nbt.child("SpawnX")?.unwrap_i32()?,
            level_nbt.child("SpawnY")?.unwrap_i32()?,
            level_nbt.child("SpawnZ")?.unwrap_i32()?,
        );
        let time = AtomicU64::new(level_nbt.child("Time")?.unwrap_i64()? as u64);
        dimensions.insert(0, Arc::new(VanillaWorld::new(DimensionType::Overworld, &directory.join("region"))?) as Arc<dyn WorldT + 'static>);
        //TODO: nether & end loading
        Ok(VanillaLevel {
            directory: directory.to_path_buf(),
            dimensions,
            border_settings,
            day_time,
            game_rules,
            seed,
            spawn,
            time,
        })
    }
}

impl LevelT for VanillaLevel {
    fn dimensions(&self) -> &CHashMap<i32, World> {
        &self.dimensions
    }

    fn get_border_settings(&self) -> &BorderSettings {
        &self.border_settings
    }

    fn day_time(&self) -> u64 {
        self.day_time.load(Ordering::Relaxed)
    }

    fn game_rules(&self) -> &CHashMap<String, String> {
        &self.game_rules
    }

    fn seed(&self) -> u64 {
        self.seed
    }

    fn spawn(&self) -> (i32, i32, i32) {
        self.spawn
    }

    fn time(&self) -> u64 {
        self.time.load(Ordering::Relaxed)
    }

}