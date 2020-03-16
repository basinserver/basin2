#![allow(non_snake_case)]
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate basin2_lib;

pub mod blocks;
pub mod materials;
pub mod loot_table;
pub mod items;
pub mod item_stack;
pub mod mob_effects;
pub mod recipes;
pub mod loader;
pub mod entities;

use basin2_lib::Registry;
pub use items::{ Item, ItemT, FoodProperties };
pub use blocks::{ Block, BlockT, BlockProperty };
pub use mob_effects::{ MobEffect, MobEffectT };
pub use materials::{ Material, MaterialT };
pub use item_stack::ItemStack;
pub use recipes::{ RecipeSerializer, SimpleCookingSerializer };
pub use loader::*;
pub use entities::{ EntityType, EntityTypeT };
use std::sync::Arc;

lazy_static! {
    pub static ref BLOCKS: Registry<BlockT> = { let mut registry = Registry::new(); blocks::construct_registry(&mut registry); registry };
    pub static ref ITEMS: Registry<ItemT> = { let _ = *BLOCKS; let mut registry = Registry::new(); items::construct_registry(&mut registry); registry };
    pub static ref MOB_EFFECTS: Registry<MobEffectT> = { let mut registry = Registry::new(); mob_effects::construct_registry(&mut registry); registry };
    pub static ref ENTITY_TYPES: Registry<EntityTypeT> = { let mut registry = Registry::new(); entities::construct_registry(&mut registry); registry };

    pub static ref DATA: Arc<Data> = { Arc::new(Data::load().expect("failed to load data")) };
}

pub fn load() {
    let _ = *BLOCKS;
    let _ = *ITEMS;
    let _ = *MOB_EFFECTS;
    let _ = *DATA;

}