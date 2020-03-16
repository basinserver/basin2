use crate::world::level::{ LevelT, BorderSettings };
use crate::world::{ World, WorldT };
use super::VanillaWorld;
use chashmap::CHashMap;
use std::sync::atomic::{ AtomicU64, AtomicU32, Ordering };
use basin2_lib::nbt::*;
use basin2_lib::result::*;
use basin2_lib::Atomic;
use std::path::{ Path, PathBuf };
use std::fs;
use bytes::BytesMut;
use std::sync::Arc;
use flate2::write::{GzDecoder};
use std::io::Write;
use basin2_protocol::network::{ DimensionType, Difficulty };
use uuid::Uuid;
use log::*;
use std::convert::TryFrom;
use crate::util::CONFIG;

pub struct VanillaLevel {
    directory: PathBuf,
    dimensions: CHashMap<i32, World>,
    border_settings: BorderSettings,
    day_time: AtomicU64,
    game_rules: CHashMap<String, String>,
    seed: u64,
    spawn: (i32, i32, i32),
    time: AtomicU64,
    player_data: CHashMap<Uuid, Nbt>,
    difficulty: Atomic<Difficulty>,
    difficulty_locked: bool,
    next_entity_id: AtomicU32,
}

fn gzip_read_nbt<T: AsRef<Path>>(file: T) -> Result<Nbt> {
    let raw_nbt = BytesMut::from(&fs::read(file)?[..]);
    let mut deflater = GzDecoder::new(vec![]);
    deflater.write_all(&raw_nbt)?;
    let mut raw_nbt = BytesMut::from(&deflater.finish()?[..]);

    Nbt::parse(&mut raw_nbt)
}

impl VanillaLevel {
    pub fn new<T: AsRef<Path>>(directory: T) -> Result<Arc<VanillaLevel>> {
        let directory = directory.as_ref();
        let level_file = directory.join("level.dat");
        let dimensions = CHashMap::new();

        let level_nbt = gzip_read_nbt(level_file)?;
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

        let player_data = CHashMap::new();
        let player_data_dir = directory.join("playerdata/");
        for player_file in player_data_dir.read_dir()? {
            if let Ok(player_file) = player_file {
                let filename = player_file.file_name();
                let filename = filename.to_str().unwrap(); // uuid.dat
                let player_file = player_file.path();
                if let Ok(uuid) = scan_fmt!(filename, "{}.dat", String) {
                    let player_nbt = gzip_read_nbt(player_file)?;

                    player_data.insert(Uuid::parse_str(&uuid)?, player_nbt);
                } else {
                    warn!("unexpected playerdata filename, skipping: {}", filename);
                    continue;
                }
            }
        }

        let level = Arc::new(VanillaLevel {
            directory: directory.to_path_buf(),
            dimensions,
            border_settings,
            day_time,
            game_rules,
            seed,
            spawn,
            time,
            player_data,
            difficulty: Atomic::from(Difficulty::try_from(&*CONFIG.difficulty)?),
            difficulty_locked: CONFIG.difficulty_locked,
            next_entity_id: AtomicU32::new(1),
        });

        level.dimensions.insert(0, Arc::new(VanillaWorld::new(level.clone(), DimensionType::Overworld, &directory.join("region"))?) as Arc<dyn WorldT + 'static>);
        //TODO: nether & end loading

        Ok(level)
    }
}

impl LevelT for VanillaLevel {
    fn dimensions(&self) -> &CHashMap<i32, World> {
        &self.dimensions
    }

    fn player_data(&self, uuid: &Uuid) -> Option<Nbt> {
        self.player_data.get(uuid).map(|data| data.clone())
    }

    fn set_player_data(&self, uuid: &Uuid, player_data: &Nbt) {
        self.player_data.insert(uuid.clone(), player_data.clone());
    }

    fn difficulty(&self) -> (Difficulty, bool) {
        (self.difficulty.get(), self.difficulty_locked)
    }

    fn set_difficulty(&self, difficulty: Difficulty) {
        self.difficulty.set(difficulty);
    }

    fn next_entity_id(&self) -> u32 {
        self.next_entity_id.fetch_add(1, Ordering::Relaxed)
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