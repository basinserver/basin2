use basin2_lib::ilib::{ Registry, RegistryItem, AtomicSet };
use std::sync::{ Arc, atomic::AtomicU32, atomic::Ordering };

#[derive(Debug)]
pub struct MobEffectT {
    pub registry_name: AtomicSet<String>,
    pub registry_id: AtomicU32,
}

#[derive(Debug)]
pub struct MobEffectInstance {
    pub mob_effect: MobEffect,
    pub duration: u32,
    pub amplifier: u32,
}

impl RegistryItem for MobEffectT {
    fn registered(&self, key: &str, id: u32) {
        self.registry_name.try_set(key.to_string());
        self.registry_id.compare_and_swap(0, id, Ordering::Relaxed);
    }
}

impl MobEffectT {
    pub fn instance(self: &Arc<Self>, duration: u32, amplifier: u32) -> MobEffectInstance {
        MobEffectInstance { mob_effect: self.clone(), duration, amplifier }
    }
}

pub fn construct_registry(registry: &mut Registry<MobEffectT>) {
    registry.insert("null", Arc::new(MobEffectT { registry_name: AtomicSet::new(), registry_id: AtomicU32::new(0) })); // 1-based index hack
    registry.insert("speed", MOVEMENT_SPEED.clone());
    registry.insert("slowness", MOVEMENT_SLOWDOWN.clone());
    registry.insert("haste", DIG_SPEED.clone());
    registry.insert("mining_fatigue", DIG_SLOWDOWN.clone());
    registry.insert("strength", DAMAGE_BOOST.clone());
    registry.insert("instant_health", HEAL.clone());
    registry.insert("instant_damage", HARM.clone());
    registry.insert("jump_boost", JUMP.clone());
    registry.insert("nausea", CONFUSION.clone());
    registry.insert("regeneration", REGENERATION.clone());
    registry.insert("resistance", DAMAGE_RESISTANCE.clone());
    registry.insert("fire_resistance", FIRE_RESISTANCE.clone());
    registry.insert("water_breathing", WATER_BREATHING.clone());
    registry.insert("invisibility", INVISIBILITY.clone());
    registry.insert("blindness", BLINDNESS.clone());
    registry.insert("night_vision", NIGHT_VISION.clone());
    registry.insert("hunger", HUNGER.clone());
    registry.insert("weakness", WEAKNESS.clone());
    registry.insert("poison", POISON.clone());
    registry.insert("wither", WITHER.clone());
    registry.insert("health_boost", HEALTH_BOOST.clone());
    registry.insert("absorption", ABSORPTION.clone());
    registry.insert("saturation", SATURATION.clone());
    registry.insert("glowing", GLOWING.clone());
    registry.insert("levitation", LEVITATION.clone());
    registry.insert("luck", LUCK.clone());
    registry.insert("unluck", UNLUCK.clone());
    registry.insert("slow_falling", SLOW_FALLING.clone());
    registry.insert("conduit_power", CONDUIT_POWER.clone());
    registry.insert("dolphins_grace", DOLPHINS_GRACE.clone());
    registry.insert("bad_omen", BAD_OMEN.clone());
    registry.insert("hero_of_the_village", HERO_OF_THE_VILLAGE.clone());    
}

pub type MobEffect = Arc<MobEffectT>;

lazy_static! {
    pub static ref MOVEMENT_SPEED: MobEffect = { Arc::new(MobEffectT { registry_name: AtomicSet::new(), registry_id: AtomicU32::new(0) }) };
    pub static ref MOVEMENT_SLOWDOWN: MobEffect = { Arc::new(MobEffectT { registry_name: AtomicSet::new(), registry_id: AtomicU32::new(0) }) };
    pub static ref DIG_SPEED: MobEffect = { Arc::new(MobEffectT { registry_name: AtomicSet::new(), registry_id: AtomicU32::new(0) }) };
    pub static ref DIG_SLOWDOWN: MobEffect = { Arc::new(MobEffectT { registry_name: AtomicSet::new(), registry_id: AtomicU32::new(0) }) };
    pub static ref DAMAGE_BOOST: MobEffect = { Arc::new(MobEffectT { registry_name: AtomicSet::new(), registry_id: AtomicU32::new(0) }) };
    pub static ref HEAL: MobEffect = { Arc::new(MobEffectT { registry_name: AtomicSet::new(), registry_id: AtomicU32::new(0) }) };
    pub static ref HARM: MobEffect = { Arc::new(MobEffectT { registry_name: AtomicSet::new(), registry_id: AtomicU32::new(0) }) };
    pub static ref JUMP: MobEffect = { Arc::new(MobEffectT { registry_name: AtomicSet::new(), registry_id: AtomicU32::new(0) }) };
    pub static ref CONFUSION: MobEffect = { Arc::new(MobEffectT { registry_name: AtomicSet::new(), registry_id: AtomicU32::new(0) }) };
    pub static ref REGENERATION: MobEffect = { Arc::new(MobEffectT { registry_name: AtomicSet::new(), registry_id: AtomicU32::new(0) }) };
    pub static ref DAMAGE_RESISTANCE: MobEffect = { Arc::new(MobEffectT { registry_name: AtomicSet::new(), registry_id: AtomicU32::new(0) }) };
    pub static ref FIRE_RESISTANCE: MobEffect = { Arc::new(MobEffectT { registry_name: AtomicSet::new(), registry_id: AtomicU32::new(0) }) };
    pub static ref WATER_BREATHING: MobEffect = { Arc::new(MobEffectT { registry_name: AtomicSet::new(), registry_id: AtomicU32::new(0) }) };
    pub static ref INVISIBILITY: MobEffect = { Arc::new(MobEffectT { registry_name: AtomicSet::new(), registry_id: AtomicU32::new(0) }) };
    pub static ref BLINDNESS: MobEffect = { Arc::new(MobEffectT { registry_name: AtomicSet::new(), registry_id: AtomicU32::new(0) }) };
    pub static ref NIGHT_VISION: MobEffect = { Arc::new(MobEffectT { registry_name: AtomicSet::new(), registry_id: AtomicU32::new(0) }) };
    pub static ref HUNGER: MobEffect = { Arc::new(MobEffectT { registry_name: AtomicSet::new(), registry_id: AtomicU32::new(0) }) };
    pub static ref WEAKNESS: MobEffect = { Arc::new(MobEffectT { registry_name: AtomicSet::new(), registry_id: AtomicU32::new(0) }) };
    pub static ref POISON: MobEffect = { Arc::new(MobEffectT { registry_name: AtomicSet::new(), registry_id: AtomicU32::new(0) }) };
    pub static ref WITHER: MobEffect = { Arc::new(MobEffectT { registry_name: AtomicSet::new(), registry_id: AtomicU32::new(0) }) };
    pub static ref HEALTH_BOOST: MobEffect = { Arc::new(MobEffectT { registry_name: AtomicSet::new(), registry_id: AtomicU32::new(0) }) };
    pub static ref ABSORPTION: MobEffect = { Arc::new(MobEffectT { registry_name: AtomicSet::new(), registry_id: AtomicU32::new(0) }) };
    pub static ref SATURATION: MobEffect = { Arc::new(MobEffectT { registry_name: AtomicSet::new(), registry_id: AtomicU32::new(0) }) };
    pub static ref GLOWING: MobEffect = { Arc::new(MobEffectT { registry_name: AtomicSet::new(), registry_id: AtomicU32::new(0) }) };
    pub static ref LEVITATION: MobEffect = { Arc::new(MobEffectT { registry_name: AtomicSet::new(), registry_id: AtomicU32::new(0) }) };
    pub static ref LUCK: MobEffect = { Arc::new(MobEffectT { registry_name: AtomicSet::new(), registry_id: AtomicU32::new(0) }) };
    pub static ref UNLUCK: MobEffect = { Arc::new(MobEffectT { registry_name: AtomicSet::new(), registry_id: AtomicU32::new(0) }) };
    pub static ref SLOW_FALLING: MobEffect = { Arc::new(MobEffectT { registry_name: AtomicSet::new(), registry_id: AtomicU32::new(0) }) };
    pub static ref CONDUIT_POWER: MobEffect = { Arc::new(MobEffectT { registry_name: AtomicSet::new(), registry_id: AtomicU32::new(0) }) };
    pub static ref DOLPHINS_GRACE: MobEffect = { Arc::new(MobEffectT { registry_name: AtomicSet::new(), registry_id: AtomicU32::new(0) }) };
    pub static ref BAD_OMEN: MobEffect = { Arc::new(MobEffectT { registry_name: AtomicSet::new(), registry_id: AtomicU32::new(0) }) };
    pub static ref HERO_OF_THE_VILLAGE: MobEffect = { Arc::new(MobEffectT { registry_name: AtomicSet::new(), registry_id: AtomicU32::new(0) }) };
}