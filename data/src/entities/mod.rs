use basin2_lib::result::*;
use basin2_lib::{AtomicSet, Nbt, Registry, RegistryItem};
use linked_hash_map::LinkedHashMap;
use std::any::Any;
use std::fmt::Debug;
use std::sync::{atomic::AtomicU32, atomic::Ordering, Arc};

pub trait EntityData: Any + Send + Sync + 'static {}

impl EntityData for () {}

pub trait EntityTypeData: Send + Sync + Debug + 'static {
    fn parse(&self, nbt: &Nbt) -> Result<Box<dyn EntityData>>;
    fn serialize(&self, data: &dyn EntityData) -> Result<Nbt>;
}

impl EntityTypeData for () {
    fn parse(&self, nbt: &Nbt) -> Result<Box<dyn EntityData>> {
        Ok(Box::new(()))
    }

    fn serialize(&self, data: &dyn EntityData) -> Result<Nbt> {
        Ok(Nbt::Compound {
            children: LinkedHashMap::new(),
        })
    }
}

#[derive(Debug)]
pub struct EntityTypeT {
    pub registry_name: AtomicSet<String>,
    pub registry_id: AtomicU32,
    pub class_name: Option<String>,
    pub should_save: bool,
    pub should_summon: bool,
    pub fire_immune: bool,
    pub can_spawn_far_away: bool,
    pub dimensions: (f32, f32),
    pub data_manager: Box<dyn EntityTypeData>,
}

impl Default for EntityTypeT {
    fn default() -> EntityTypeT {
        EntityTypeT {
            registry_name: AtomicSet::new(),
            registry_id: AtomicU32::new(0),
            class_name: None,
            should_save: true,
            should_summon: true,
            fire_immune: false,
            can_spawn_far_away: true,
            dimensions: (0.6, 1.8),
            data_manager: Box::new(()),
        }
    }
}

impl RegistryItem for EntityTypeT {
    fn registered(&self, key: &str, id: u32) {
        self.registry_name.try_set(key.to_string());
        self.registry_id.compare_and_swap(0, id, Ordering::Relaxed);
    }
}

pub type EntityType = Arc<EntityTypeT>;

lazy_static! {
    pub static ref AREA_EFFECT_CLOUD: EntityType = {
        Arc::new(EntityTypeT {
            class_name: Some("AreaEffectCloud".to_string()),
            fire_immune: true,
            dimensions: (6.00, 0.50),
            ..Default::default()
        })
    };
}
lazy_static! {
    pub static ref ARMOR_STAND: EntityType = {
        Arc::new(EntityTypeT {
            class_name: Some("ArmorStand".to_string()),
            dimensions: (0.50, 1.98),
            ..Default::default()
        })
    };
}
lazy_static! {
    pub static ref ARROW: EntityType = {
        Arc::new(EntityTypeT {
            class_name: Some("Arrow".to_string()),
            dimensions: (0.50, 0.50),
            ..Default::default()
        })
    };
}
lazy_static! {
    pub static ref BAT: EntityType = {
        Arc::new(EntityTypeT {
            class_name: Some("Bat".to_string()),
            dimensions: (0.50, 0.90),
            ..Default::default()
        })
    };
}
lazy_static! {
    pub static ref BEE: EntityType = {
        Arc::new(EntityTypeT {
            class_name: Some("Bee".to_string()),
            dimensions: (0.70, 0.60),
            ..Default::default()
        })
    };
}
lazy_static! {
    pub static ref BLAZE: EntityType = {
        Arc::new(EntityTypeT {
            class_name: Some("Blaze".to_string()),
            fire_immune: true,
            dimensions: (0.60, 1.80),
            ..Default::default()
        })
    };
}
lazy_static! {
    pub static ref BOAT: EntityType = {
        Arc::new(EntityTypeT {
            class_name: Some("Boat".to_string()),
            dimensions: (1.38, 0.56),
            ..Default::default()
        })
    };
}
lazy_static! {
    pub static ref CAT: EntityType = {
        Arc::new(EntityTypeT {
            class_name: Some("Cat".to_string()),
            dimensions: (0.60, 0.70),
            ..Default::default()
        })
    };
}
lazy_static! {
    pub static ref CAVE_SPIDER: EntityType = {
        Arc::new(EntityTypeT {
            class_name: Some("CaveSpider".to_string()),
            dimensions: (0.70, 0.50),
            ..Default::default()
        })
    };
}
lazy_static! {
    pub static ref CHICKEN: EntityType = {
        Arc::new(EntityTypeT {
            class_name: Some("Chicken".to_string()),
            dimensions: (0.40, 0.70),
            ..Default::default()
        })
    };
}
lazy_static! {
    pub static ref COD: EntityType = {
        Arc::new(EntityTypeT {
            class_name: Some("Cod".to_string()),
            dimensions: (0.50, 0.30),
            ..Default::default()
        })
    };
}
lazy_static! {
    pub static ref COW: EntityType = {
        Arc::new(EntityTypeT {
            class_name: Some("Cow".to_string()),
            dimensions: (0.90, 1.40),
            ..Default::default()
        })
    };
}
lazy_static! {
    pub static ref CREEPER: EntityType = {
        Arc::new(EntityTypeT {
            class_name: Some("Creeper".to_string()),
            dimensions: (0.60, 1.70),
            ..Default::default()
        })
    };
}
lazy_static! {
    pub static ref DONKEY: EntityType = {
        Arc::new(EntityTypeT {
            class_name: Some("Donkey".to_string()),
            dimensions: (1.40, 1.50),
            ..Default::default()
        })
    };
}
lazy_static! {
    pub static ref DOLPHIN: EntityType = {
        Arc::new(EntityTypeT {
            class_name: Some("Dolphin".to_string()),
            dimensions: (0.90, 0.60),
            ..Default::default()
        })
    };
}
lazy_static! {
    pub static ref DRAGON_FIREBALL: EntityType = {
        Arc::new(EntityTypeT {
            class_name: Some("DragonFireball".to_string()),
            dimensions: (1.00, 1.00),
            ..Default::default()
        })
    };
}
lazy_static! {
    pub static ref DROWNED: EntityType = {
        Arc::new(EntityTypeT {
            class_name: Some("Drowned".to_string()),
            dimensions: (0.60, 1.95),
            ..Default::default()
        })
    };
}
lazy_static! {
    pub static ref ELDER_GUARDIAN: EntityType = {
        Arc::new(EntityTypeT {
            class_name: Some("ElderGuardian".to_string()),
            dimensions: (2.00, 2.00),
            ..Default::default()
        })
    };
}
lazy_static! {
    pub static ref END_CRYSTAL: EntityType = {
        Arc::new(EntityTypeT {
            class_name: Some("EndCrystal".to_string()),
            dimensions: (2.00, 2.00),
            ..Default::default()
        })
    };
}
lazy_static! {
    pub static ref ENDER_DRAGON: EntityType = {
        Arc::new(EntityTypeT {
            class_name: Some("EnderDragon".to_string()),
            fire_immune: true,
            dimensions: (16.00, 8.00),
            ..Default::default()
        })
    };
}
lazy_static! {
    pub static ref ENDERMAN: EntityType = {
        Arc::new(EntityTypeT {
            class_name: Some("EnderMan".to_string()),
            dimensions: (0.60, 2.90),
            ..Default::default()
        })
    };
}
lazy_static! {
    pub static ref ENDERMITE: EntityType = {
        Arc::new(EntityTypeT {
            class_name: Some("Endermite".to_string()),
            dimensions: (0.40, 0.30),
            ..Default::default()
        })
    };
}
lazy_static! {
    pub static ref EVOKER_FANGS: EntityType = {
        Arc::new(EntityTypeT {
            class_name: Some("EvokerFangs".to_string()),
            dimensions: (0.50, 0.80),
            ..Default::default()
        })
    };
}
lazy_static! {
    pub static ref EVOKER: EntityType = {
        Arc::new(EntityTypeT {
            class_name: Some("Evoker".to_string()),
            dimensions: (0.60, 1.95),
            ..Default::default()
        })
    };
}
lazy_static! {
    pub static ref EXPERIENCE_ORB: EntityType = {
        Arc::new(EntityTypeT {
            class_name: Some("ExperienceOrb".to_string()),
            dimensions: (0.50, 0.50),
            ..Default::default()
        })
    };
}
lazy_static! {
    pub static ref EYE_OF_ENDER: EntityType = {
        Arc::new(EntityTypeT {
            class_name: Some("EyeOfEnder".to_string()),
            dimensions: (0.25, 0.25),
            ..Default::default()
        })
    };
}
lazy_static! {
    pub static ref FALLING_BLOCK: EntityType = {
        Arc::new(EntityTypeT {
            class_name: Some("FallingBlockEntity".to_string()),
            dimensions: (0.98, 0.98),
            ..Default::default()
        })
    };
}
lazy_static! {
    pub static ref FIREWORK_ROCKET: EntityType = {
        Arc::new(EntityTypeT {
            class_name: Some("FireworkRocketEntity".to_string()),
            dimensions: (0.25, 0.25),
            ..Default::default()
        })
    };
}
lazy_static! {
    pub static ref FOX: EntityType = {
        Arc::new(EntityTypeT {
            class_name: Some("Fox".to_string()),
            dimensions: (0.60, 0.70),
            ..Default::default()
        })
    };
}
lazy_static! {
    pub static ref GHAST: EntityType = {
        Arc::new(EntityTypeT {
            class_name: Some("Ghast".to_string()),
            fire_immune: true,
            dimensions: (4.00, 4.00),
            ..Default::default()
        })
    };
}
lazy_static! {
    pub static ref GIANT: EntityType = {
        Arc::new(EntityTypeT {
            class_name: Some("Giant".to_string()),
            dimensions: (3.60, 12.00),
            ..Default::default()
        })
    };
}
lazy_static! {
    pub static ref GUARDIAN: EntityType = {
        Arc::new(EntityTypeT {
            class_name: Some("Guardian".to_string()),
            dimensions: (0.85, 0.85),
            ..Default::default()
        })
    };
}
lazy_static! {
    pub static ref HORSE: EntityType = {
        Arc::new(EntityTypeT {
            class_name: Some("Horse".to_string()),
            dimensions: (1.40, 1.60),
            ..Default::default()
        })
    };
}
lazy_static! {
    pub static ref HUSK: EntityType = {
        Arc::new(EntityTypeT {
            class_name: Some("Husk".to_string()),
            dimensions: (0.60, 1.95),
            ..Default::default()
        })
    };
}
lazy_static! {
    pub static ref ILLUSIONER: EntityType = {
        Arc::new(EntityTypeT {
            class_name: Some("Illusioner".to_string()),
            dimensions: (0.60, 1.95),
            ..Default::default()
        })
    };
}
lazy_static! {
    pub static ref ITEM: EntityType = {
        Arc::new(EntityTypeT {
            class_name: Some("ItemEntity".to_string()),
            dimensions: (0.25, 0.25),
            ..Default::default()
        })
    };
}
lazy_static! {
    pub static ref ITEM_FRAME: EntityType = {
        Arc::new(EntityTypeT {
            class_name: Some("ItemFrame".to_string()),
            dimensions: (0.50, 0.50),
            ..Default::default()
        })
    };
}
lazy_static! {
    pub static ref FIREBALL: EntityType = {
        Arc::new(EntityTypeT {
            class_name: Some("LargeFireball".to_string()),
            dimensions: (1.00, 1.00),
            ..Default::default()
        })
    };
}
lazy_static! {
    pub static ref LEASH_KNOT: EntityType = {
        Arc::new(EntityTypeT {
            class_name: Some("LeashFenceKnotEntity".to_string()),
            should_save: false,
            dimensions: (0.50, 0.50),
            ..Default::default()
        })
    };
}
lazy_static! {
    pub static ref LLAMA: EntityType = {
        Arc::new(EntityTypeT {
            class_name: Some("Llama".to_string()),
            dimensions: (0.90, 1.87),
            ..Default::default()
        })
    };
}
lazy_static! {
    pub static ref LLAMA_SPIT: EntityType = {
        Arc::new(EntityTypeT {
            class_name: Some("LlamaSpit".to_string()),
            dimensions: (0.25, 0.25),
            ..Default::default()
        })
    };
}
lazy_static! {
    pub static ref MAGMA_CUBE: EntityType = {
        Arc::new(EntityTypeT {
            class_name: Some("MagmaCube".to_string()),
            fire_immune: true,
            dimensions: (2.04, 2.04),
            ..Default::default()
        })
    };
}
lazy_static! {
    pub static ref MINECART: EntityType = {
        Arc::new(EntityTypeT {
            class_name: Some("Minecart".to_string()),
            dimensions: (0.98, 0.70),
            ..Default::default()
        })
    };
}
lazy_static! {
    pub static ref CHEST_MINECART: EntityType = {
        Arc::new(EntityTypeT {
            class_name: Some("MinecartChest".to_string()),
            dimensions: (0.98, 0.70),
            ..Default::default()
        })
    };
}
lazy_static! {
    pub static ref COMMAND_BLOCK_MINECART: EntityType = {
        Arc::new(EntityTypeT {
            class_name: Some("MinecartCommandBlock".to_string()),
            dimensions: (0.98, 0.70),
            ..Default::default()
        })
    };
}
lazy_static! {
    pub static ref FURNACE_MINECART: EntityType = {
        Arc::new(EntityTypeT {
            class_name: Some("MinecartFurnace".to_string()),
            dimensions: (0.98, 0.70),
            ..Default::default()
        })
    };
}
lazy_static! {
    pub static ref HOPPER_MINECART: EntityType = {
        Arc::new(EntityTypeT {
            class_name: Some("MinecartHopper".to_string()),
            dimensions: (0.98, 0.70),
            ..Default::default()
        })
    };
}
lazy_static! {
    pub static ref SPAWNER_MINECART: EntityType = {
        Arc::new(EntityTypeT {
            class_name: Some("MinecartSpawner".to_string()),
            dimensions: (0.98, 0.70),
            ..Default::default()
        })
    };
}
lazy_static! {
    pub static ref TNT_MINECART: EntityType = {
        Arc::new(EntityTypeT {
            class_name: Some("MinecartTNT".to_string()),
            dimensions: (0.98, 0.70),
            ..Default::default()
        })
    };
}
lazy_static! {
    pub static ref MULE: EntityType = {
        Arc::new(EntityTypeT {
            class_name: Some("Mule".to_string()),
            dimensions: (1.40, 1.60),
            ..Default::default()
        })
    };
}
lazy_static! {
    pub static ref MOOSHROOM: EntityType = {
        Arc::new(EntityTypeT {
            class_name: Some("MushroomCow".to_string()),
            dimensions: (0.90, 1.40),
            ..Default::default()
        })
    };
}
lazy_static! {
    pub static ref OCELOT: EntityType = {
        Arc::new(EntityTypeT {
            class_name: Some("Ocelot".to_string()),
            dimensions: (0.60, 0.70),
            ..Default::default()
        })
    };
}
lazy_static! {
    pub static ref PAINTING: EntityType = {
        Arc::new(EntityTypeT {
            class_name: Some("Painting".to_string()),
            dimensions: (0.50, 0.50),
            ..Default::default()
        })
    };
}
lazy_static! {
    pub static ref PANDA: EntityType = {
        Arc::new(EntityTypeT {
            class_name: Some("Panda".to_string()),
            dimensions: (1.30, 1.25),
            ..Default::default()
        })
    };
}
lazy_static! {
    pub static ref PARROT: EntityType = {
        Arc::new(EntityTypeT {
            class_name: Some("Parrot".to_string()),
            dimensions: (0.50, 0.90),
            ..Default::default()
        })
    };
}
lazy_static! {
    pub static ref PIG: EntityType = {
        Arc::new(EntityTypeT {
            class_name: Some("Pig".to_string()),
            dimensions: (0.90, 0.90),
            ..Default::default()
        })
    };
}
lazy_static! {
    pub static ref PUFFERFISH: EntityType = {
        Arc::new(EntityTypeT {
            class_name: Some("Pufferfish".to_string()),
            dimensions: (0.70, 0.70),
            ..Default::default()
        })
    };
}
lazy_static! {
    pub static ref ZOMBIE_PIGMAN: EntityType = {
        Arc::new(EntityTypeT {
            class_name: Some("PigZombie".to_string()),
            fire_immune: true,
            dimensions: (0.60, 1.95),
            ..Default::default()
        })
    };
}
lazy_static! {
    pub static ref POLAR_BEAR: EntityType = {
        Arc::new(EntityTypeT {
            class_name: Some("PolarBear".to_string()),
            dimensions: (1.40, 1.40),
            ..Default::default()
        })
    };
}
lazy_static! {
    pub static ref TNT: EntityType = {
        Arc::new(EntityTypeT {
            class_name: Some("PrimedTnt".to_string()),
            fire_immune: true,
            dimensions: (0.98, 0.98),
            ..Default::default()
        })
    };
}
lazy_static! {
    pub static ref RABBIT: EntityType = {
        Arc::new(EntityTypeT {
            class_name: Some("Rabbit".to_string()),
            dimensions: (0.40, 0.50),
            ..Default::default()
        })
    };
}
lazy_static! {
    pub static ref SALMON: EntityType = {
        Arc::new(EntityTypeT {
            class_name: Some("Salmon".to_string()),
            dimensions: (0.70, 0.40),
            ..Default::default()
        })
    };
}
lazy_static! {
    pub static ref SHEEP: EntityType = {
        Arc::new(EntityTypeT {
            class_name: Some("Sheep".to_string()),
            dimensions: (0.90, 1.30),
            ..Default::default()
        })
    };
}
lazy_static! {
    pub static ref SHULKER: EntityType = {
        Arc::new(EntityTypeT {
            class_name: Some("Shulker".to_string()),
            fire_immune: true,
            can_spawn_far_away: true,
            dimensions: (1.00, 1.00),
            ..Default::default()
        })
    };
}
lazy_static! {
    pub static ref SHULKER_BULLET: EntityType = {
        Arc::new(EntityTypeT {
            class_name: Some("ShulkerBullet".to_string()),
            dimensions: (0.31, 0.31),
            ..Default::default()
        })
    };
}
lazy_static! {
    pub static ref SILVERFISH: EntityType = {
        Arc::new(EntityTypeT {
            class_name: Some("Silverfish".to_string()),
            dimensions: (0.40, 0.30),
            ..Default::default()
        })
    };
}
lazy_static! {
    pub static ref SKELETON: EntityType = {
        Arc::new(EntityTypeT {
            class_name: Some("Skeleton".to_string()),
            dimensions: (0.60, 1.99),
            ..Default::default()
        })
    };
}
lazy_static! {
    pub static ref SKELETON_HORSE: EntityType = {
        Arc::new(EntityTypeT {
            class_name: Some("SkeletonHorse".to_string()),
            dimensions: (1.40, 1.60),
            ..Default::default()
        })
    };
}
lazy_static! {
    pub static ref SLIME: EntityType = {
        Arc::new(EntityTypeT {
            class_name: Some("Slime".to_string()),
            dimensions: (2.04, 2.04),
            ..Default::default()
        })
    };
}
lazy_static! {
    pub static ref SMALL_FIREBALL: EntityType = {
        Arc::new(EntityTypeT {
            class_name: Some("SmallFireball".to_string()),
            dimensions: (0.31, 0.31),
            ..Default::default()
        })
    };
}
lazy_static! {
    pub static ref SNOW_GOLEM: EntityType = {
        Arc::new(EntityTypeT {
            class_name: Some("SnowGolem".to_string()),
            dimensions: (0.70, 1.90),
            ..Default::default()
        })
    };
}
lazy_static! {
    pub static ref SNOWBALL: EntityType = {
        Arc::new(EntityTypeT {
            class_name: Some("Snowball".to_string()),
            dimensions: (0.25, 0.25),
            ..Default::default()
        })
    };
}
lazy_static! {
    pub static ref SPECTRAL_ARROW: EntityType = {
        Arc::new(EntityTypeT {
            class_name: Some("SpectralArrow".to_string()),
            dimensions: (0.50, 0.50),
            ..Default::default()
        })
    };
}
lazy_static! {
    pub static ref SPIDER: EntityType = {
        Arc::new(EntityTypeT {
            class_name: Some("Spider".to_string()),
            dimensions: (1.40, 0.90),
            ..Default::default()
        })
    };
}
lazy_static! {
    pub static ref SQUID: EntityType = {
        Arc::new(EntityTypeT {
            class_name: Some("Squid".to_string()),
            dimensions: (0.80, 0.80),
            ..Default::default()
        })
    };
}
lazy_static! {
    pub static ref STRAY: EntityType = {
        Arc::new(EntityTypeT {
            class_name: Some("Stray".to_string()),
            dimensions: (0.60, 1.99),
            ..Default::default()
        })
    };
}
lazy_static! {
    pub static ref TRADER_LLAMA: EntityType = {
        Arc::new(EntityTypeT {
            class_name: Some("TraderLlama".to_string()),
            dimensions: (0.90, 1.87),
            ..Default::default()
        })
    };
}
lazy_static! {
    pub static ref TROPICAL_FISH: EntityType = {
        Arc::new(EntityTypeT {
            class_name: Some("TropicalFish".to_string()),
            dimensions: (0.50, 0.40),
            ..Default::default()
        })
    };
}
lazy_static! {
    pub static ref TURTLE: EntityType = {
        Arc::new(EntityTypeT {
            class_name: Some("Turtle".to_string()),
            dimensions: (1.20, 0.40),
            ..Default::default()
        })
    };
}
lazy_static! {
    pub static ref EGG: EntityType = {
        Arc::new(EntityTypeT {
            class_name: Some("ThrownEgg".to_string()),
            dimensions: (0.25, 0.25),
            ..Default::default()
        })
    };
}
lazy_static! {
    pub static ref ENDER_PEARL: EntityType = {
        Arc::new(EntityTypeT {
            class_name: Some("ThrownEnderpearl".to_string()),
            dimensions: (0.25, 0.25),
            ..Default::default()
        })
    };
}
lazy_static! {
    pub static ref EXPERIENCE_BOTTLE: EntityType = {
        Arc::new(EntityTypeT {
            class_name: Some("ThrownExperienceBottle".to_string()),
            dimensions: (0.25, 0.25),
            ..Default::default()
        })
    };
}
lazy_static! {
    pub static ref POTION: EntityType = {
        Arc::new(EntityTypeT {
            class_name: Some("ThrownPotion".to_string()),
            dimensions: (0.25, 0.25),
            ..Default::default()
        })
    };
}
lazy_static! {
    pub static ref TRIDENT: EntityType = {
        Arc::new(EntityTypeT {
            class_name: Some("ThrownTrident".to_string()),
            dimensions: (0.50, 0.50),
            ..Default::default()
        })
    };
}
lazy_static! {
    pub static ref VEX: EntityType = {
        Arc::new(EntityTypeT {
            class_name: Some("Vex".to_string()),
            fire_immune: true,
            dimensions: (0.40, 0.80),
            ..Default::default()
        })
    };
}
lazy_static! {
    pub static ref VILLAGER: EntityType = {
        Arc::new(EntityTypeT {
            class_name: Some("Villager".to_string()),
            dimensions: (0.60, 1.95),
            ..Default::default()
        })
    };
}
lazy_static! {
    pub static ref IRON_GOLEM: EntityType = {
        Arc::new(EntityTypeT {
            class_name: Some("IronGolem".to_string()),
            dimensions: (1.40, 2.70),
            ..Default::default()
        })
    };
}
lazy_static! {
    pub static ref VINDICATOR: EntityType = {
        Arc::new(EntityTypeT {
            class_name: Some("Vindicator".to_string()),
            dimensions: (0.60, 1.95),
            ..Default::default()
        })
    };
}
lazy_static! {
    pub static ref PILLAGER: EntityType = {
        Arc::new(EntityTypeT {
            class_name: Some("Pillager".to_string()),
            can_spawn_far_away: true,
            dimensions: (0.60, 1.95),
            ..Default::default()
        })
    };
}
lazy_static! {
    pub static ref WANDERING_TRADER: EntityType = {
        Arc::new(EntityTypeT {
            class_name: Some("WanderingTrader".to_string()),
            dimensions: (0.60, 1.95),
            ..Default::default()
        })
    };
}
lazy_static! {
    pub static ref WITCH: EntityType = {
        Arc::new(EntityTypeT {
            class_name: Some("Witch".to_string()),
            dimensions: (0.60, 1.95),
            ..Default::default()
        })
    };
}
lazy_static! {
    pub static ref WITHER: EntityType = {
        Arc::new(EntityTypeT {
            class_name: Some("WitherBoss".to_string()),
            fire_immune: true,
            dimensions: (0.90, 3.50),
            ..Default::default()
        })
    };
}
lazy_static! {
    pub static ref WITHER_SKELETON: EntityType = {
        Arc::new(EntityTypeT {
            class_name: Some("WitherSkeleton".to_string()),
            fire_immune: true,
            dimensions: (0.70, 2.40),
            ..Default::default()
        })
    };
}
lazy_static! {
    pub static ref WITHER_SKULL: EntityType = {
        Arc::new(EntityTypeT {
            class_name: Some("WitherSkull".to_string()),
            dimensions: (0.31, 0.31),
            ..Default::default()
        })
    };
}
lazy_static! {
    pub static ref WOLF: EntityType = {
        Arc::new(EntityTypeT {
            class_name: Some("Wolf".to_string()),
            dimensions: (0.60, 0.85),
            ..Default::default()
        })
    };
}
lazy_static! {
    pub static ref ZOMBIE: EntityType = {
        Arc::new(EntityTypeT {
            class_name: Some("Zombie".to_string()),
            dimensions: (0.60, 1.95),
            ..Default::default()
        })
    };
}
lazy_static! {
    pub static ref ZOMBIE_HORSE: EntityType = {
        Arc::new(EntityTypeT {
            class_name: Some("ZombieHorse".to_string()),
            dimensions: (1.40, 1.60),
            ..Default::default()
        })
    };
}
lazy_static! {
    pub static ref ZOMBIE_VILLAGER: EntityType = {
        Arc::new(EntityTypeT {
            class_name: Some("ZombieVillager".to_string()),
            dimensions: (0.60, 1.95),
            ..Default::default()
        })
    };
}
lazy_static! {
    pub static ref PHANTOM: EntityType = {
        Arc::new(EntityTypeT {
            class_name: Some("Phantom".to_string()),
            dimensions: (0.90, 0.50),
            ..Default::default()
        })
    };
}
lazy_static! {
    pub static ref RAVAGER: EntityType = {
        Arc::new(EntityTypeT {
            class_name: Some("Ravager".to_string()),
            dimensions: (1.95, 2.20),
            ..Default::default()
        })
    };
}
lazy_static! {
    pub static ref LIGHTNING_BOLT: EntityType = {
        Arc::new(EntityTypeT {
            class_name: None,
            should_save: false,
            dimensions: (0.00, 0.00),
            ..Default::default()
        })
    };
}
lazy_static! {
    pub static ref PLAYER: EntityType = {
        Arc::new(EntityTypeT {
            class_name: None,
            should_save: false,
            should_summon: false,
            dimensions: (0.60, 1.80),
            ..Default::default()
        })
    };
}
lazy_static! {
    pub static ref FISHING_BOBBER: EntityType = {
        Arc::new(EntityTypeT {
            class_name: None,
            should_save: false,
            should_summon: false,
            dimensions: (0.25, 0.25),
            ..Default::default()
        })
    };
}

pub fn construct_registry(registry: &mut Registry<EntityTypeT>) {
    registry.insert("area_effect_cloud", AREA_EFFECT_CLOUD.clone());
    registry.insert("armor_stand", ARMOR_STAND.clone());
    registry.insert("arrow", ARROW.clone());
    registry.insert("bat", BAT.clone());
    registry.insert("bee", BEE.clone());
    registry.insert("blaze", BLAZE.clone());
    registry.insert("boat", BOAT.clone());
    registry.insert("cat", CAT.clone());
    registry.insert("cave_spider", CAVE_SPIDER.clone());
    registry.insert("chicken", CHICKEN.clone());
    registry.insert("cod", COD.clone());
    registry.insert("cow", COW.clone());
    registry.insert("creeper", CREEPER.clone());
    registry.insert("donkey", DONKEY.clone());
    registry.insert("dolphin", DOLPHIN.clone());
    registry.insert("dragon_fireball", DRAGON_FIREBALL.clone());
    registry.insert("drowned", DROWNED.clone());
    registry.insert("elder_guardian", ELDER_GUARDIAN.clone());
    registry.insert("end_crystal", END_CRYSTAL.clone());
    registry.insert("ender_dragon", ENDER_DRAGON.clone());
    registry.insert("enderman", ENDERMAN.clone());
    registry.insert("endermite", ENDERMITE.clone());
    registry.insert("evoker_fangs", EVOKER_FANGS.clone());
    registry.insert("evoker", EVOKER.clone());
    registry.insert("experience_orb", EXPERIENCE_ORB.clone());
    registry.insert("eye_of_ender", EYE_OF_ENDER.clone());
    registry.insert("falling_block", FALLING_BLOCK.clone());
    registry.insert("firework_rocket", FIREWORK_ROCKET.clone());
    registry.insert("fox", FOX.clone());
    registry.insert("ghast", GHAST.clone());
    registry.insert("giant", GIANT.clone());
    registry.insert("guardian", GUARDIAN.clone());
    registry.insert("horse", HORSE.clone());
    registry.insert("husk", HUSK.clone());
    registry.insert("illusioner", ILLUSIONER.clone());
    registry.insert("item", ITEM.clone());
    registry.insert("item_frame", ITEM_FRAME.clone());
    registry.insert("fireball", FIREBALL.clone());
    registry.insert("leash_knot", LEASH_KNOT.clone());
    registry.insert("llama", LLAMA.clone());
    registry.insert("llama_spit", LLAMA_SPIT.clone());
    registry.insert("magma_cube", MAGMA_CUBE.clone());
    registry.insert("minecart", MINECART.clone());
    registry.insert("chest_minecart", CHEST_MINECART.clone());
    registry.insert("command_block_minecart", COMMAND_BLOCK_MINECART.clone());
    registry.insert("furnace_minecart", FURNACE_MINECART.clone());
    registry.insert("hopper_minecart", HOPPER_MINECART.clone());
    registry.insert("spawner_minecart", SPAWNER_MINECART.clone());
    registry.insert("tnt_minecart", TNT_MINECART.clone());
    registry.insert("mule", MULE.clone());
    registry.insert("mooshroom", MOOSHROOM.clone());
    registry.insert("ocelot", OCELOT.clone());
    registry.insert("painting", PAINTING.clone());
    registry.insert("panda", PANDA.clone());
    registry.insert("parrot", PARROT.clone());
    registry.insert("pig", PIG.clone());
    registry.insert("pufferfish", PUFFERFISH.clone());
    registry.insert("zombie_pigman", ZOMBIE_PIGMAN.clone());
    registry.insert("polar_bear", POLAR_BEAR.clone());
    registry.insert("tnt", TNT.clone());
    registry.insert("rabbit", RABBIT.clone());
    registry.insert("salmon", SALMON.clone());
    registry.insert("sheep", SHEEP.clone());
    registry.insert("shulker", SHULKER.clone());
    registry.insert("shulker_bullet", SHULKER_BULLET.clone());
    registry.insert("silverfish", SILVERFISH.clone());
    registry.insert("skeleton", SKELETON.clone());
    registry.insert("skeleton_horse", SKELETON_HORSE.clone());
    registry.insert("slime", SLIME.clone());
    registry.insert("small_fireball", SMALL_FIREBALL.clone());
    registry.insert("snow_golem", SNOW_GOLEM.clone());
    registry.insert("snowball", SNOWBALL.clone());
    registry.insert("spectral_arrow", SPECTRAL_ARROW.clone());
    registry.insert("spider", SPIDER.clone());
    registry.insert("squid", SQUID.clone());
    registry.insert("stray", STRAY.clone());
    registry.insert("trader_llama", TRADER_LLAMA.clone());
    registry.insert("tropical_fish", TROPICAL_FISH.clone());
    registry.insert("turtle", TURTLE.clone());
    registry.insert("egg", EGG.clone());
    registry.insert("ender_pearl", ENDER_PEARL.clone());
    registry.insert("experience_bottle", EXPERIENCE_BOTTLE.clone());
    registry.insert("potion", POTION.clone());
    registry.insert("trident", TRIDENT.clone());
    registry.insert("vex", VEX.clone());
    registry.insert("villager", VILLAGER.clone());
    registry.insert("iron_golem", IRON_GOLEM.clone());
    registry.insert("vindicator", VINDICATOR.clone());
    registry.insert("pillager", PILLAGER.clone());
    registry.insert("wandering_trader", WANDERING_TRADER.clone());
    registry.insert("witch", WITCH.clone());
    registry.insert("wither", WITHER.clone());
    registry.insert("wither_skeleton", WITHER_SKELETON.clone());
    registry.insert("wither_skull", WITHER_SKULL.clone());
    registry.insert("wolf", WOLF.clone());
    registry.insert("zombie", ZOMBIE.clone());
    registry.insert("zombie_horse", ZOMBIE_HORSE.clone());
    registry.insert("zombie_villager", ZOMBIE_VILLAGER.clone());
    registry.insert("phantom", PHANTOM.clone());
    registry.insert("ravager", RAVAGER.clone());
    registry.insert("lightning_bolt", LIGHTNING_BOLT.clone());
    registry.insert("player", PLAYER.clone());
    registry.insert("fishing_bobber", FISHING_BOBBER.clone());
}
