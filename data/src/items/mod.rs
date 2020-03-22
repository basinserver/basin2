use super::blocks::{self, Block};
use super::mob_effects::MobEffectInstance;
use crate::ITEMS;
use basin2_lib::result::*;
use basin2_lib::{AtomicSet, Registry, RegistryItem};
use std::convert::TryFrom;
use std::sync::{atomic::AtomicU32, atomic::Ordering, Arc};

mod data;
pub use data::*;

#[derive(Debug)]
pub struct FoodProperties {
    pub nutrition: i32,
    pub saturation_mod: f32,
    pub is_meat: bool,
    pub can_always_eat: bool,
    pub fast_food: bool,
    pub effects: Vec<(MobEffectInstance, f32)>,
}

#[derive(Debug)]
pub struct ItemT {
    pub registry_name: AtomicSet<String>,
    pub registry_id: AtomicU32,
    pub max_stack_size: i32,
    pub max_damage: i32,
    pub crafting_remaining_item: Option<Item>,
    pub food_properties: Option<FoodProperties>,
}

impl PartialEq for ItemT {
    fn eq(&self, other: &ItemT) -> bool {
        return self.clone().registry_name == other.clone().registry_name;
    }
}

pub type Item = Arc<ItemT>;

impl RegistryItem for ItemT {
    fn registered(&self, key: &str, id: u32) {
        self.registry_name.try_set(key.to_string());
        self.registry_id.compare_and_swap(0, id, Ordering::Relaxed);
    }
}

impl From<Block> for ItemT {
    fn from(block: Block) -> ItemT {
        ItemT {
            registry_name: block.registry_name.clone(),
            registry_id: AtomicU32::new(0),
            max_stack_size: 64,
            max_damage: 0,
            crafting_remaining_item: None,
            food_properties: None,
        }
    }
}

impl ItemT {
    fn from_block_stack_size(block: Block, max_stack_size: i32) -> Item {
        Arc::new(ItemT {
            registry_name: block.registry_name.clone(),
            registry_id: AtomicU32::new(0),
            max_stack_size: max_stack_size,
            max_damage: 0,
            crafting_remaining_item: None,
            food_properties: None,
        })
    }

    pub fn try_from(key: &str) -> Result<Item> {
        ITEMS
            .get_str(key)
            .ok_or(basin_err!("couldn't find item: {}", key))
    }
}

pub fn construct_registry(registry: &mut Registry<ItemT>) {
    registry.insert(&blocks::AIR.registry_name.clone(), AIR.clone());
    registry.insert(&blocks::STONE.registry_name.clone(), STONE.clone());
    registry.insert(&blocks::GRANITE.registry_name.clone(), GRANITE.clone());
    registry.insert(
        &blocks::POLISHED_GRANITE.registry_name.clone(),
        POLISHED_GRANITE.clone(),
    );
    registry.insert(&blocks::DIORITE.registry_name.clone(), DIORITE.clone());
    registry.insert(
        &blocks::POLISHED_DIORITE.registry_name.clone(),
        POLISHED_DIORITE.clone(),
    );
    registry.insert(&blocks::ANDESITE.registry_name.clone(), ANDESITE.clone());
    registry.insert(
        &blocks::POLISHED_ANDESITE.registry_name.clone(),
        POLISHED_ANDESITE.clone(),
    );
    registry.insert(
        &blocks::GRASS_BLOCK.registry_name.clone(),
        GRASS_BLOCK.clone(),
    );
    registry.insert(&blocks::DIRT.registry_name.clone(), DIRT.clone());
    registry.insert(
        &blocks::COARSE_DIRT.registry_name.clone(),
        COARSE_DIRT.clone(),
    );
    registry.insert(&blocks::PODZOL.registry_name.clone(), PODZOL.clone());
    registry.insert(
        &blocks::COBBLESTONE.registry_name.clone(),
        COBBLESTONE.clone(),
    );
    registry.insert(
        &blocks::OAK_PLANKS.registry_name.clone(),
        OAK_PLANKS.clone(),
    );
    registry.insert(
        &blocks::SPRUCE_PLANKS.registry_name.clone(),
        SPRUCE_PLANKS.clone(),
    );
    registry.insert(
        &blocks::BIRCH_PLANKS.registry_name.clone(),
        BIRCH_PLANKS.clone(),
    );
    registry.insert(
        &blocks::JUNGLE_PLANKS.registry_name.clone(),
        JUNGLE_PLANKS.clone(),
    );
    registry.insert(
        &blocks::ACACIA_PLANKS.registry_name.clone(),
        ACACIA_PLANKS.clone(),
    );
    registry.insert(
        &blocks::DARK_OAK_PLANKS.registry_name.clone(),
        DARK_OAK_PLANKS.clone(),
    );
    registry.insert(
        &blocks::OAK_SAPLING.registry_name.clone(),
        OAK_SAPLING.clone(),
    );
    registry.insert(
        &blocks::SPRUCE_SAPLING.registry_name.clone(),
        SPRUCE_SAPLING.clone(),
    );
    registry.insert(
        &blocks::BIRCH_SAPLING.registry_name.clone(),
        BIRCH_SAPLING.clone(),
    );
    registry.insert(
        &blocks::JUNGLE_SAPLING.registry_name.clone(),
        JUNGLE_SAPLING.clone(),
    );
    registry.insert(
        &blocks::ACACIA_SAPLING.registry_name.clone(),
        ACACIA_SAPLING.clone(),
    );
    registry.insert(
        &blocks::DARK_OAK_SAPLING.registry_name.clone(),
        DARK_OAK_SAPLING.clone(),
    );
    registry.insert(&blocks::BEDROCK.registry_name.clone(), BEDROCK.clone());
    registry.insert(&blocks::SAND.registry_name.clone(), SAND.clone());
    registry.insert(&blocks::RED_SAND.registry_name.clone(), RED_SAND.clone());
    registry.insert(&blocks::GRAVEL.registry_name.clone(), GRAVEL.clone());
    registry.insert(&blocks::GOLD_ORE.registry_name.clone(), GOLD_ORE.clone());
    registry.insert(&blocks::IRON_ORE.registry_name.clone(), IRON_ORE.clone());
    registry.insert(&blocks::COAL_ORE.registry_name.clone(), COAL_ORE.clone());
    registry.insert(&blocks::OAK_LOG.registry_name.clone(), OAK_LOG.clone());
    registry.insert(
        &blocks::SPRUCE_LOG.registry_name.clone(),
        SPRUCE_LOG.clone(),
    );
    registry.insert(&blocks::BIRCH_LOG.registry_name.clone(), BIRCH_LOG.clone());
    registry.insert(
        &blocks::JUNGLE_LOG.registry_name.clone(),
        JUNGLE_LOG.clone(),
    );
    registry.insert(
        &blocks::ACACIA_LOG.registry_name.clone(),
        ACACIA_LOG.clone(),
    );
    registry.insert(
        &blocks::DARK_OAK_LOG.registry_name.clone(),
        DARK_OAK_LOG.clone(),
    );
    registry.insert(
        &blocks::STRIPPED_OAK_LOG.registry_name.clone(),
        STRIPPED_OAK_LOG.clone(),
    );
    registry.insert(
        &blocks::STRIPPED_SPRUCE_LOG.registry_name.clone(),
        STRIPPED_SPRUCE_LOG.clone(),
    );
    registry.insert(
        &blocks::STRIPPED_BIRCH_LOG.registry_name.clone(),
        STRIPPED_BIRCH_LOG.clone(),
    );
    registry.insert(
        &blocks::STRIPPED_JUNGLE_LOG.registry_name.clone(),
        STRIPPED_JUNGLE_LOG.clone(),
    );
    registry.insert(
        &blocks::STRIPPED_ACACIA_LOG.registry_name.clone(),
        STRIPPED_ACACIA_LOG.clone(),
    );
    registry.insert(
        &blocks::STRIPPED_DARK_OAK_LOG.registry_name.clone(),
        STRIPPED_DARK_OAK_LOG.clone(),
    );
    registry.insert(
        &blocks::STRIPPED_OAK_WOOD.registry_name.clone(),
        STRIPPED_OAK_WOOD.clone(),
    );
    registry.insert(
        &blocks::STRIPPED_SPRUCE_WOOD.registry_name.clone(),
        STRIPPED_SPRUCE_WOOD.clone(),
    );
    registry.insert(
        &blocks::STRIPPED_BIRCH_WOOD.registry_name.clone(),
        STRIPPED_BIRCH_WOOD.clone(),
    );
    registry.insert(
        &blocks::STRIPPED_JUNGLE_WOOD.registry_name.clone(),
        STRIPPED_JUNGLE_WOOD.clone(),
    );
    registry.insert(
        &blocks::STRIPPED_ACACIA_WOOD.registry_name.clone(),
        STRIPPED_ACACIA_WOOD.clone(),
    );
    registry.insert(
        &blocks::STRIPPED_DARK_OAK_WOOD.registry_name.clone(),
        STRIPPED_DARK_OAK_WOOD.clone(),
    );
    registry.insert(&blocks::OAK_WOOD.registry_name.clone(), OAK_WOOD.clone());
    registry.insert(
        &blocks::SPRUCE_WOOD.registry_name.clone(),
        SPRUCE_WOOD.clone(),
    );
    registry.insert(
        &blocks::BIRCH_WOOD.registry_name.clone(),
        BIRCH_WOOD.clone(),
    );
    registry.insert(
        &blocks::JUNGLE_WOOD.registry_name.clone(),
        JUNGLE_WOOD.clone(),
    );
    registry.insert(
        &blocks::ACACIA_WOOD.registry_name.clone(),
        ACACIA_WOOD.clone(),
    );
    registry.insert(
        &blocks::DARK_OAK_WOOD.registry_name.clone(),
        DARK_OAK_WOOD.clone(),
    );
    registry.insert(
        &blocks::OAK_LEAVES.registry_name.clone(),
        OAK_LEAVES.clone(),
    );
    registry.insert(
        &blocks::SPRUCE_LEAVES.registry_name.clone(),
        SPRUCE_LEAVES.clone(),
    );
    registry.insert(
        &blocks::BIRCH_LEAVES.registry_name.clone(),
        BIRCH_LEAVES.clone(),
    );
    registry.insert(
        &blocks::JUNGLE_LEAVES.registry_name.clone(),
        JUNGLE_LEAVES.clone(),
    );
    registry.insert(
        &blocks::ACACIA_LEAVES.registry_name.clone(),
        ACACIA_LEAVES.clone(),
    );
    registry.insert(
        &blocks::DARK_OAK_LEAVES.registry_name.clone(),
        DARK_OAK_LEAVES.clone(),
    );
    registry.insert(&blocks::SPONGE.registry_name.clone(), SPONGE.clone());
    registry.insert(
        &blocks::WET_SPONGE.registry_name.clone(),
        WET_SPONGE.clone(),
    );
    registry.insert(&blocks::GLASS.registry_name.clone(), GLASS.clone());
    registry.insert(&blocks::LAPIS_ORE.registry_name.clone(), LAPIS_ORE.clone());
    registry.insert(
        &blocks::LAPIS_BLOCK.registry_name.clone(),
        LAPIS_BLOCK.clone(),
    );
    registry.insert(&blocks::DISPENSER.registry_name.clone(), DISPENSER.clone());
    registry.insert(&blocks::SANDSTONE.registry_name.clone(), SANDSTONE.clone());
    registry.insert(
        &blocks::CHISELED_SANDSTONE.registry_name.clone(),
        CHISELED_SANDSTONE.clone(),
    );
    registry.insert(
        &blocks::CUT_SANDSTONE.registry_name.clone(),
        CUT_SANDSTONE.clone(),
    );
    registry.insert(
        &blocks::NOTE_BLOCK.registry_name.clone(),
        NOTE_BLOCK.clone(),
    );
    registry.insert(
        &blocks::POWERED_RAIL.registry_name.clone(),
        POWERED_RAIL.clone(),
    );
    registry.insert(
        &blocks::DETECTOR_RAIL.registry_name.clone(),
        DETECTOR_RAIL.clone(),
    );
    registry.insert(
        &blocks::STICKY_PISTON.registry_name.clone(),
        STICKY_PISTON.clone(),
    );
    registry.insert(&blocks::COBWEB.registry_name.clone(), COBWEB.clone());
    registry.insert(&blocks::GRASS.registry_name.clone(), GRASS.clone());
    registry.insert(&blocks::FERN.registry_name.clone(), FERN.clone());
    registry.insert(&blocks::DEAD_BUSH.registry_name.clone(), DEAD_BUSH.clone());
    registry.insert(&blocks::SEAGRASS.registry_name.clone(), SEAGRASS.clone());
    registry.insert(
        &blocks::SEA_PICKLE.registry_name.clone(),
        SEA_PICKLE.clone(),
    );
    registry.insert(&blocks::PISTON.registry_name.clone(), PISTON.clone());
    registry.insert(
        &blocks::WHITE_WOOL.registry_name.clone(),
        WHITE_WOOL.clone(),
    );
    registry.insert(
        &blocks::ORANGE_WOOL.registry_name.clone(),
        ORANGE_WOOL.clone(),
    );
    registry.insert(
        &blocks::MAGENTA_WOOL.registry_name.clone(),
        MAGENTA_WOOL.clone(),
    );
    registry.insert(
        &blocks::LIGHT_BLUE_WOOL.registry_name.clone(),
        LIGHT_BLUE_WOOL.clone(),
    );
    registry.insert(
        &blocks::YELLOW_WOOL.registry_name.clone(),
        YELLOW_WOOL.clone(),
    );
    registry.insert(&blocks::LIME_WOOL.registry_name.clone(), LIME_WOOL.clone());
    registry.insert(&blocks::PINK_WOOL.registry_name.clone(), PINK_WOOL.clone());
    registry.insert(&blocks::GRAY_WOOL.registry_name.clone(), GRAY_WOOL.clone());
    registry.insert(
        &blocks::LIGHT_GRAY_WOOL.registry_name.clone(),
        LIGHT_GRAY_WOOL.clone(),
    );
    registry.insert(&blocks::CYAN_WOOL.registry_name.clone(), CYAN_WOOL.clone());
    registry.insert(
        &blocks::PURPLE_WOOL.registry_name.clone(),
        PURPLE_WOOL.clone(),
    );
    registry.insert(&blocks::BLUE_WOOL.registry_name.clone(), BLUE_WOOL.clone());
    registry.insert(
        &blocks::BROWN_WOOL.registry_name.clone(),
        BROWN_WOOL.clone(),
    );
    registry.insert(
        &blocks::GREEN_WOOL.registry_name.clone(),
        GREEN_WOOL.clone(),
    );
    registry.insert(&blocks::RED_WOOL.registry_name.clone(), RED_WOOL.clone());
    registry.insert(
        &blocks::BLACK_WOOL.registry_name.clone(),
        BLACK_WOOL.clone(),
    );
    registry.insert(&blocks::DANDELION.registry_name.clone(), DANDELION.clone());
    registry.insert(&blocks::POPPY.registry_name.clone(), POPPY.clone());
    registry.insert(
        &blocks::BLUE_ORCHID.registry_name.clone(),
        BLUE_ORCHID.clone(),
    );
    registry.insert(&blocks::ALLIUM.registry_name.clone(), ALLIUM.clone());
    registry.insert(
        &blocks::AZURE_BLUET.registry_name.clone(),
        AZURE_BLUET.clone(),
    );
    registry.insert(&blocks::RED_TULIP.registry_name.clone(), RED_TULIP.clone());
    registry.insert(
        &blocks::ORANGE_TULIP.registry_name.clone(),
        ORANGE_TULIP.clone(),
    );
    registry.insert(
        &blocks::WHITE_TULIP.registry_name.clone(),
        WHITE_TULIP.clone(),
    );
    registry.insert(
        &blocks::PINK_TULIP.registry_name.clone(),
        PINK_TULIP.clone(),
    );
    registry.insert(
        &blocks::OXEYE_DAISY.registry_name.clone(),
        OXEYE_DAISY.clone(),
    );
    registry.insert(
        &blocks::CORNFLOWER.registry_name.clone(),
        CORNFLOWER.clone(),
    );
    registry.insert(
        &blocks::LILY_OF_THE_VALLEY.registry_name.clone(),
        LILY_OF_THE_VALLEY.clone(),
    );
    registry.insert(
        &blocks::WITHER_ROSE.registry_name.clone(),
        WITHER_ROSE.clone(),
    );
    registry.insert(
        &blocks::BROWN_MUSHROOM.registry_name.clone(),
        BROWN_MUSHROOM.clone(),
    );
    registry.insert(
        &blocks::RED_MUSHROOM.registry_name.clone(),
        RED_MUSHROOM.clone(),
    );
    registry.insert(
        &blocks::GOLD_BLOCK.registry_name.clone(),
        GOLD_BLOCK.clone(),
    );
    registry.insert(
        &blocks::IRON_BLOCK.registry_name.clone(),
        IRON_BLOCK.clone(),
    );
    registry.insert(&blocks::OAK_SLAB.registry_name.clone(), OAK_SLAB.clone());
    registry.insert(
        &blocks::SPRUCE_SLAB.registry_name.clone(),
        SPRUCE_SLAB.clone(),
    );
    registry.insert(
        &blocks::BIRCH_SLAB.registry_name.clone(),
        BIRCH_SLAB.clone(),
    );
    registry.insert(
        &blocks::JUNGLE_SLAB.registry_name.clone(),
        JUNGLE_SLAB.clone(),
    );
    registry.insert(
        &blocks::ACACIA_SLAB.registry_name.clone(),
        ACACIA_SLAB.clone(),
    );
    registry.insert(
        &blocks::DARK_OAK_SLAB.registry_name.clone(),
        DARK_OAK_SLAB.clone(),
    );
    registry.insert(
        &blocks::STONE_SLAB.registry_name.clone(),
        STONE_SLAB.clone(),
    );
    registry.insert(
        &blocks::SMOOTH_STONE_SLAB.registry_name.clone(),
        SMOOTH_STONE_SLAB.clone(),
    );
    registry.insert(
        &blocks::SANDSTONE_SLAB.registry_name.clone(),
        SANDSTONE_SLAB.clone(),
    );
    registry.insert(
        &blocks::CUT_SANDSTONE_SLAB.registry_name.clone(),
        CUT_SANDSTONE_SLAB.clone(),
    );
    registry.insert(
        &blocks::PETRIFIED_OAK_SLAB.registry_name.clone(),
        PETRIFIED_OAK_SLAB.clone(),
    );
    registry.insert(
        &blocks::COBBLESTONE_SLAB.registry_name.clone(),
        COBBLESTONE_SLAB.clone(),
    );
    registry.insert(
        &blocks::BRICK_SLAB.registry_name.clone(),
        BRICK_SLAB.clone(),
    );
    registry.insert(
        &blocks::STONE_BRICK_SLAB.registry_name.clone(),
        STONE_BRICK_SLAB.clone(),
    );
    registry.insert(
        &blocks::NETHER_BRICK_SLAB.registry_name.clone(),
        NETHER_BRICK_SLAB.clone(),
    );
    registry.insert(
        &blocks::QUARTZ_SLAB.registry_name.clone(),
        QUARTZ_SLAB.clone(),
    );
    registry.insert(
        &blocks::RED_SANDSTONE_SLAB.registry_name.clone(),
        RED_SANDSTONE_SLAB.clone(),
    );
    registry.insert(
        &blocks::CUT_RED_SANDSTONE_SLAB.registry_name.clone(),
        CUT_RED_SANDSTONE_SLAB.clone(),
    );
    registry.insert(
        &blocks::PURPUR_SLAB.registry_name.clone(),
        PURPUR_SLAB.clone(),
    );
    registry.insert(
        &blocks::PRISMARINE_SLAB.registry_name.clone(),
        PRISMARINE_SLAB.clone(),
    );
    registry.insert(
        &blocks::PRISMARINE_BRICK_SLAB.registry_name.clone(),
        PRISMARINE_BRICK_SLAB.clone(),
    );
    registry.insert(
        &blocks::DARK_PRISMARINE_SLAB.registry_name.clone(),
        DARK_PRISMARINE_SLAB.clone(),
    );
    registry.insert(
        &blocks::SMOOTH_QUARTZ.registry_name.clone(),
        SMOOTH_QUARTZ.clone(),
    );
    registry.insert(
        &blocks::SMOOTH_RED_SANDSTONE.registry_name.clone(),
        SMOOTH_RED_SANDSTONE.clone(),
    );
    registry.insert(
        &blocks::SMOOTH_SANDSTONE.registry_name.clone(),
        SMOOTH_SANDSTONE.clone(),
    );
    registry.insert(
        &blocks::SMOOTH_STONE.registry_name.clone(),
        SMOOTH_STONE.clone(),
    );
    registry.insert(&blocks::BRICKS.registry_name.clone(), BRICKS.clone());
    registry.insert(&blocks::TNT.registry_name.clone(), TNT.clone());
    registry.insert(&blocks::BOOKSHELF.registry_name.clone(), BOOKSHELF.clone());
    registry.insert(
        &blocks::MOSSY_COBBLESTONE.registry_name.clone(),
        MOSSY_COBBLESTONE.clone(),
    );
    registry.insert(&blocks::OBSIDIAN.registry_name.clone(), OBSIDIAN.clone());
    registry.insert(&blocks::TORCH.registry_name.clone(), TORCH.clone());
    registry.insert(&blocks::END_ROD.registry_name.clone(), END_ROD.clone());
    registry.insert(
        &blocks::CHORUS_PLANT.registry_name.clone(),
        CHORUS_PLANT.clone(),
    );
    registry.insert(
        &blocks::CHORUS_FLOWER.registry_name.clone(),
        CHORUS_FLOWER.clone(),
    );
    registry.insert(
        &blocks::PURPUR_BLOCK.registry_name.clone(),
        PURPUR_BLOCK.clone(),
    );
    registry.insert(
        &blocks::PURPUR_PILLAR.registry_name.clone(),
        PURPUR_PILLAR.clone(),
    );
    registry.insert(
        &blocks::PURPUR_STAIRS.registry_name.clone(),
        PURPUR_STAIRS.clone(),
    );
    registry.insert(&blocks::SPAWNER.registry_name.clone(), SPAWNER.clone());
    registry.insert(
        &blocks::OAK_STAIRS.registry_name.clone(),
        OAK_STAIRS.clone(),
    );
    registry.insert(&blocks::CHEST.registry_name.clone(), CHEST.clone());
    registry.insert(
        &blocks::DIAMOND_ORE.registry_name.clone(),
        DIAMOND_ORE.clone(),
    );
    registry.insert(
        &blocks::DIAMOND_BLOCK.registry_name.clone(),
        DIAMOND_BLOCK.clone(),
    );
    registry.insert(
        &blocks::CRAFTING_TABLE.registry_name.clone(),
        CRAFTING_TABLE.clone(),
    );
    registry.insert(&blocks::FARMLAND.registry_name.clone(), FARMLAND.clone());
    registry.insert(&blocks::FURNACE.registry_name.clone(), FURNACE.clone());
    registry.insert(&blocks::LADDER.registry_name.clone(), LADDER.clone());
    registry.insert(&blocks::RAIL.registry_name.clone(), RAIL.clone());
    registry.insert(
        &blocks::COBBLESTONE_STAIRS.registry_name.clone(),
        COBBLESTONE_STAIRS.clone(),
    );
    registry.insert(&blocks::LEVER.registry_name.clone(), LEVER.clone());
    registry.insert(
        &blocks::STONE_PRESSURE_PLATE.registry_name.clone(),
        STONE_PRESSURE_PLATE.clone(),
    );
    registry.insert(
        &blocks::OAK_PRESSURE_PLATE.registry_name.clone(),
        OAK_PRESSURE_PLATE.clone(),
    );
    registry.insert(
        &blocks::SPRUCE_PRESSURE_PLATE.registry_name.clone(),
        SPRUCE_PRESSURE_PLATE.clone(),
    );
    registry.insert(
        &blocks::BIRCH_PRESSURE_PLATE.registry_name.clone(),
        BIRCH_PRESSURE_PLATE.clone(),
    );
    registry.insert(
        &blocks::JUNGLE_PRESSURE_PLATE.registry_name.clone(),
        JUNGLE_PRESSURE_PLATE.clone(),
    );
    registry.insert(
        &blocks::ACACIA_PRESSURE_PLATE.registry_name.clone(),
        ACACIA_PRESSURE_PLATE.clone(),
    );
    registry.insert(
        &blocks::DARK_OAK_PRESSURE_PLATE.registry_name.clone(),
        DARK_OAK_PRESSURE_PLATE.clone(),
    );
    registry.insert(
        &blocks::REDSTONE_ORE.registry_name.clone(),
        REDSTONE_ORE.clone(),
    );
    registry.insert(
        &blocks::REDSTONE_TORCH.registry_name.clone(),
        REDSTONE_TORCH.clone(),
    );
    registry.insert(
        &blocks::STONE_BUTTON.registry_name.clone(),
        STONE_BUTTON.clone(),
    );
    registry.insert(&blocks::SNOW.registry_name.clone(), SNOW.clone());
    registry.insert(&blocks::ICE.registry_name.clone(), ICE.clone());
    registry.insert(
        &blocks::SNOW_BLOCK.registry_name.clone(),
        SNOW_BLOCK.clone(),
    );
    registry.insert(&blocks::CACTUS.registry_name.clone(), CACTUS.clone());
    registry.insert(&blocks::CLAY.registry_name.clone(), CLAY.clone());
    registry.insert(&blocks::JUKEBOX.registry_name.clone(), JUKEBOX.clone());
    registry.insert(&blocks::OAK_FENCE.registry_name.clone(), OAK_FENCE.clone());
    registry.insert(
        &blocks::SPRUCE_FENCE.registry_name.clone(),
        SPRUCE_FENCE.clone(),
    );
    registry.insert(
        &blocks::BIRCH_FENCE.registry_name.clone(),
        BIRCH_FENCE.clone(),
    );
    registry.insert(
        &blocks::JUNGLE_FENCE.registry_name.clone(),
        JUNGLE_FENCE.clone(),
    );
    registry.insert(
        &blocks::ACACIA_FENCE.registry_name.clone(),
        ACACIA_FENCE.clone(),
    );
    registry.insert(
        &blocks::DARK_OAK_FENCE.registry_name.clone(),
        DARK_OAK_FENCE.clone(),
    );
    registry.insert(&blocks::PUMPKIN.registry_name.clone(), PUMPKIN.clone());
    registry.insert(
        &blocks::CARVED_PUMPKIN.registry_name.clone(),
        CARVED_PUMPKIN.clone(),
    );
    registry.insert(
        &blocks::NETHERRACK.registry_name.clone(),
        NETHERRACK.clone(),
    );
    registry.insert(&blocks::SOUL_SAND.registry_name.clone(), SOUL_SAND.clone());
    registry.insert(&blocks::GLOWSTONE.registry_name.clone(), GLOWSTONE.clone());
    registry.insert(
        &blocks::JACK_O_LANTERN.registry_name.clone(),
        JACK_O_LANTERN.clone(),
    );
    registry.insert(
        &blocks::OAK_TRAPDOOR.registry_name.clone(),
        OAK_TRAPDOOR.clone(),
    );
    registry.insert(
        &blocks::SPRUCE_TRAPDOOR.registry_name.clone(),
        SPRUCE_TRAPDOOR.clone(),
    );
    registry.insert(
        &blocks::BIRCH_TRAPDOOR.registry_name.clone(),
        BIRCH_TRAPDOOR.clone(),
    );
    registry.insert(
        &blocks::JUNGLE_TRAPDOOR.registry_name.clone(),
        JUNGLE_TRAPDOOR.clone(),
    );
    registry.insert(
        &blocks::ACACIA_TRAPDOOR.registry_name.clone(),
        ACACIA_TRAPDOOR.clone(),
    );
    registry.insert(
        &blocks::DARK_OAK_TRAPDOOR.registry_name.clone(),
        DARK_OAK_TRAPDOOR.clone(),
    );
    registry.insert(
        &blocks::INFESTED_STONE.registry_name.clone(),
        INFESTED_STONE.clone(),
    );
    registry.insert(
        &blocks::INFESTED_COBBLESTONE.registry_name.clone(),
        INFESTED_COBBLESTONE.clone(),
    );
    registry.insert(
        &blocks::INFESTED_STONE_BRICKS.registry_name.clone(),
        INFESTED_STONE_BRICKS.clone(),
    );
    registry.insert(
        &blocks::INFESTED_MOSSY_STONE_BRICKS.registry_name.clone(),
        INFESTED_MOSSY_STONE_BRICKS.clone(),
    );
    registry.insert(
        &blocks::INFESTED_CRACKED_STONE_BRICKS.registry_name.clone(),
        INFESTED_CRACKED_STONE_BRICKS.clone(),
    );
    registry.insert(
        &blocks::INFESTED_CHISELED_STONE_BRICKS.registry_name.clone(),
        INFESTED_CHISELED_STONE_BRICKS.clone(),
    );
    registry.insert(
        &blocks::STONE_BRICKS.registry_name.clone(),
        STONE_BRICKS.clone(),
    );
    registry.insert(
        &blocks::MOSSY_STONE_BRICKS.registry_name.clone(),
        MOSSY_STONE_BRICKS.clone(),
    );
    registry.insert(
        &blocks::CRACKED_STONE_BRICKS.registry_name.clone(),
        CRACKED_STONE_BRICKS.clone(),
    );
    registry.insert(
        &blocks::CHISELED_STONE_BRICKS.registry_name.clone(),
        CHISELED_STONE_BRICKS.clone(),
    );
    registry.insert(
        &blocks::BROWN_MUSHROOM_BLOCK.registry_name.clone(),
        BROWN_MUSHROOM_BLOCK.clone(),
    );
    registry.insert(
        &blocks::RED_MUSHROOM_BLOCK.registry_name.clone(),
        RED_MUSHROOM_BLOCK.clone(),
    );
    registry.insert(
        &blocks::MUSHROOM_STEM.registry_name.clone(),
        MUSHROOM_STEM.clone(),
    );
    registry.insert(&blocks::IRON_BARS.registry_name.clone(), IRON_BARS.clone());
    registry.insert(
        &blocks::GLASS_PANE.registry_name.clone(),
        GLASS_PANE.clone(),
    );
    registry.insert(&blocks::MELON.registry_name.clone(), MELON.clone());
    registry.insert(&blocks::VINE.registry_name.clone(), VINE.clone());
    registry.insert(
        &blocks::OAK_FENCE_GATE.registry_name.clone(),
        OAK_FENCE_GATE.clone(),
    );
    registry.insert(
        &blocks::SPRUCE_FENCE_GATE.registry_name.clone(),
        SPRUCE_FENCE_GATE.clone(),
    );
    registry.insert(
        &blocks::BIRCH_FENCE_GATE.registry_name.clone(),
        BIRCH_FENCE_GATE.clone(),
    );
    registry.insert(
        &blocks::JUNGLE_FENCE_GATE.registry_name.clone(),
        JUNGLE_FENCE_GATE.clone(),
    );
    registry.insert(
        &blocks::ACACIA_FENCE_GATE.registry_name.clone(),
        ACACIA_FENCE_GATE.clone(),
    );
    registry.insert(
        &blocks::DARK_OAK_FENCE_GATE.registry_name.clone(),
        DARK_OAK_FENCE_GATE.clone(),
    );
    registry.insert(
        &blocks::BRICK_STAIRS.registry_name.clone(),
        BRICK_STAIRS.clone(),
    );
    registry.insert(
        &blocks::STONE_BRICK_STAIRS.registry_name.clone(),
        STONE_BRICK_STAIRS.clone(),
    );
    registry.insert(&blocks::MYCELIUM.registry_name.clone(), MYCELIUM.clone());
    registry.insert(&blocks::LILY_PAD.registry_name.clone(), LILY_PAD.clone());
    registry.insert(
        &blocks::NETHER_BRICKS.registry_name.clone(),
        NETHER_BRICKS.clone(),
    );
    registry.insert(
        &blocks::NETHER_BRICK_FENCE.registry_name.clone(),
        NETHER_BRICK_FENCE.clone(),
    );
    registry.insert(
        &blocks::NETHER_BRICK_STAIRS.registry_name.clone(),
        NETHER_BRICK_STAIRS.clone(),
    );
    registry.insert(
        &blocks::ENCHANTING_TABLE.registry_name.clone(),
        ENCHANTING_TABLE.clone(),
    );
    registry.insert(
        &blocks::END_PORTAL_FRAME.registry_name.clone(),
        END_PORTAL_FRAME.clone(),
    );
    registry.insert(&blocks::END_STONE.registry_name.clone(), END_STONE.clone());
    registry.insert(
        &blocks::END_STONE_BRICKS.registry_name.clone(),
        END_STONE_BRICKS.clone(),
    );
    registry.insert(
        &blocks::DRAGON_EGG.registry_name.clone(),
        DRAGON_EGG.clone(),
    );
    registry.insert(
        &blocks::REDSTONE_LAMP.registry_name.clone(),
        REDSTONE_LAMP.clone(),
    );
    registry.insert(
        &blocks::SANDSTONE_STAIRS.registry_name.clone(),
        SANDSTONE_STAIRS.clone(),
    );
    registry.insert(
        &blocks::EMERALD_ORE.registry_name.clone(),
        EMERALD_ORE.clone(),
    );
    registry.insert(
        &blocks::ENDER_CHEST.registry_name.clone(),
        ENDER_CHEST.clone(),
    );
    registry.insert(
        &blocks::TRIPWIRE_HOOK.registry_name.clone(),
        TRIPWIRE_HOOK.clone(),
    );
    registry.insert(
        &blocks::EMERALD_BLOCK.registry_name.clone(),
        EMERALD_BLOCK.clone(),
    );
    registry.insert(
        &blocks::SPRUCE_STAIRS.registry_name.clone(),
        SPRUCE_STAIRS.clone(),
    );
    registry.insert(
        &blocks::BIRCH_STAIRS.registry_name.clone(),
        BIRCH_STAIRS.clone(),
    );
    registry.insert(
        &blocks::JUNGLE_STAIRS.registry_name.clone(),
        JUNGLE_STAIRS.clone(),
    );
    registry.insert(
        &blocks::COMMAND_BLOCK.registry_name.clone(),
        COMMAND_BLOCK.clone(),
    );
    registry.insert(&blocks::BEACON.registry_name.clone(), BEACON.clone());
    registry.insert(
        &blocks::COBBLESTONE_WALL.registry_name.clone(),
        COBBLESTONE_WALL.clone(),
    );
    registry.insert(
        &blocks::MOSSY_COBBLESTONE_WALL.registry_name.clone(),
        MOSSY_COBBLESTONE_WALL.clone(),
    );
    registry.insert(
        &blocks::BRICK_WALL.registry_name.clone(),
        BRICK_WALL.clone(),
    );
    registry.insert(
        &blocks::PRISMARINE_WALL.registry_name.clone(),
        PRISMARINE_WALL.clone(),
    );
    registry.insert(
        &blocks::RED_SANDSTONE_WALL.registry_name.clone(),
        RED_SANDSTONE_WALL.clone(),
    );
    registry.insert(
        &blocks::MOSSY_STONE_BRICK_WALL.registry_name.clone(),
        MOSSY_STONE_BRICK_WALL.clone(),
    );
    registry.insert(
        &blocks::GRANITE_WALL.registry_name.clone(),
        GRANITE_WALL.clone(),
    );
    registry.insert(
        &blocks::STONE_BRICK_WALL.registry_name.clone(),
        STONE_BRICK_WALL.clone(),
    );
    registry.insert(
        &blocks::NETHER_BRICK_WALL.registry_name.clone(),
        NETHER_BRICK_WALL.clone(),
    );
    registry.insert(
        &blocks::ANDESITE_WALL.registry_name.clone(),
        ANDESITE_WALL.clone(),
    );
    registry.insert(
        &blocks::RED_NETHER_BRICK_WALL.registry_name.clone(),
        RED_NETHER_BRICK_WALL.clone(),
    );
    registry.insert(
        &blocks::SANDSTONE_WALL.registry_name.clone(),
        SANDSTONE_WALL.clone(),
    );
    registry.insert(
        &blocks::END_STONE_BRICK_WALL.registry_name.clone(),
        END_STONE_BRICK_WALL.clone(),
    );
    registry.insert(
        &blocks::DIORITE_WALL.registry_name.clone(),
        DIORITE_WALL.clone(),
    );
    registry.insert(
        &blocks::OAK_BUTTON.registry_name.clone(),
        OAK_BUTTON.clone(),
    );
    registry.insert(
        &blocks::SPRUCE_BUTTON.registry_name.clone(),
        SPRUCE_BUTTON.clone(),
    );
    registry.insert(
        &blocks::BIRCH_BUTTON.registry_name.clone(),
        BIRCH_BUTTON.clone(),
    );
    registry.insert(
        &blocks::JUNGLE_BUTTON.registry_name.clone(),
        JUNGLE_BUTTON.clone(),
    );
    registry.insert(
        &blocks::ACACIA_BUTTON.registry_name.clone(),
        ACACIA_BUTTON.clone(),
    );
    registry.insert(
        &blocks::DARK_OAK_BUTTON.registry_name.clone(),
        DARK_OAK_BUTTON.clone(),
    );
    registry.insert(&blocks::ANVIL.registry_name.clone(), ANVIL.clone());
    registry.insert(
        &blocks::CHIPPED_ANVIL.registry_name.clone(),
        CHIPPED_ANVIL.clone(),
    );
    registry.insert(
        &blocks::DAMAGED_ANVIL.registry_name.clone(),
        DAMAGED_ANVIL.clone(),
    );
    registry.insert(
        &blocks::TRAPPED_CHEST.registry_name.clone(),
        TRAPPED_CHEST.clone(),
    );
    registry.insert(
        &blocks::LIGHT_WEIGHTED_PRESSURE_PLATE.registry_name.clone(),
        LIGHT_WEIGHTED_PRESSURE_PLATE.clone(),
    );
    registry.insert(
        &blocks::HEAVY_WEIGHTED_PRESSURE_PLATE.registry_name.clone(),
        HEAVY_WEIGHTED_PRESSURE_PLATE.clone(),
    );
    registry.insert(
        &blocks::DAYLIGHT_DETECTOR.registry_name.clone(),
        DAYLIGHT_DETECTOR.clone(),
    );
    registry.insert(
        &blocks::REDSTONE_BLOCK.registry_name.clone(),
        REDSTONE_BLOCK.clone(),
    );
    registry.insert(
        &blocks::NETHER_QUARTZ_ORE.registry_name.clone(),
        NETHER_QUARTZ_ORE.clone(),
    );
    registry.insert(&blocks::HOPPER.registry_name.clone(), HOPPER.clone());
    registry.insert(
        &blocks::CHISELED_QUARTZ_BLOCK.registry_name.clone(),
        CHISELED_QUARTZ_BLOCK.clone(),
    );
    registry.insert(
        &blocks::QUARTZ_BLOCK.registry_name.clone(),
        QUARTZ_BLOCK.clone(),
    );
    registry.insert(
        &blocks::QUARTZ_PILLAR.registry_name.clone(),
        QUARTZ_PILLAR.clone(),
    );
    registry.insert(
        &blocks::QUARTZ_STAIRS.registry_name.clone(),
        QUARTZ_STAIRS.clone(),
    );
    registry.insert(
        &blocks::ACTIVATOR_RAIL.registry_name.clone(),
        ACTIVATOR_RAIL.clone(),
    );
    registry.insert(&blocks::DROPPER.registry_name.clone(), DROPPER.clone());
    registry.insert(
        &blocks::WHITE_TERRACOTTA.registry_name.clone(),
        WHITE_TERRACOTTA.clone(),
    );
    registry.insert(
        &blocks::ORANGE_TERRACOTTA.registry_name.clone(),
        ORANGE_TERRACOTTA.clone(),
    );
    registry.insert(
        &blocks::MAGENTA_TERRACOTTA.registry_name.clone(),
        MAGENTA_TERRACOTTA.clone(),
    );
    registry.insert(
        &blocks::LIGHT_BLUE_TERRACOTTA.registry_name.clone(),
        LIGHT_BLUE_TERRACOTTA.clone(),
    );
    registry.insert(
        &blocks::YELLOW_TERRACOTTA.registry_name.clone(),
        YELLOW_TERRACOTTA.clone(),
    );
    registry.insert(
        &blocks::LIME_TERRACOTTA.registry_name.clone(),
        LIME_TERRACOTTA.clone(),
    );
    registry.insert(
        &blocks::PINK_TERRACOTTA.registry_name.clone(),
        PINK_TERRACOTTA.clone(),
    );
    registry.insert(
        &blocks::GRAY_TERRACOTTA.registry_name.clone(),
        GRAY_TERRACOTTA.clone(),
    );
    registry.insert(
        &blocks::LIGHT_GRAY_TERRACOTTA.registry_name.clone(),
        LIGHT_GRAY_TERRACOTTA.clone(),
    );
    registry.insert(
        &blocks::CYAN_TERRACOTTA.registry_name.clone(),
        CYAN_TERRACOTTA.clone(),
    );
    registry.insert(
        &blocks::PURPLE_TERRACOTTA.registry_name.clone(),
        PURPLE_TERRACOTTA.clone(),
    );
    registry.insert(
        &blocks::BLUE_TERRACOTTA.registry_name.clone(),
        BLUE_TERRACOTTA.clone(),
    );
    registry.insert(
        &blocks::BROWN_TERRACOTTA.registry_name.clone(),
        BROWN_TERRACOTTA.clone(),
    );
    registry.insert(
        &blocks::GREEN_TERRACOTTA.registry_name.clone(),
        GREEN_TERRACOTTA.clone(),
    );
    registry.insert(
        &blocks::RED_TERRACOTTA.registry_name.clone(),
        RED_TERRACOTTA.clone(),
    );
    registry.insert(
        &blocks::BLACK_TERRACOTTA.registry_name.clone(),
        BLACK_TERRACOTTA.clone(),
    );
    registry.insert(&blocks::BARRIER.registry_name.clone(), BARRIER.clone());
    registry.insert(
        &blocks::IRON_TRAPDOOR.registry_name.clone(),
        IRON_TRAPDOOR.clone(),
    );
    registry.insert(&blocks::HAY_BLOCK.registry_name.clone(), HAY_BLOCK.clone());
    registry.insert(
        &blocks::WHITE_CARPET.registry_name.clone(),
        WHITE_CARPET.clone(),
    );
    registry.insert(
        &blocks::ORANGE_CARPET.registry_name.clone(),
        ORANGE_CARPET.clone(),
    );
    registry.insert(
        &blocks::MAGENTA_CARPET.registry_name.clone(),
        MAGENTA_CARPET.clone(),
    );
    registry.insert(
        &blocks::LIGHT_BLUE_CARPET.registry_name.clone(),
        LIGHT_BLUE_CARPET.clone(),
    );
    registry.insert(
        &blocks::YELLOW_CARPET.registry_name.clone(),
        YELLOW_CARPET.clone(),
    );
    registry.insert(
        &blocks::LIME_CARPET.registry_name.clone(),
        LIME_CARPET.clone(),
    );
    registry.insert(
        &blocks::PINK_CARPET.registry_name.clone(),
        PINK_CARPET.clone(),
    );
    registry.insert(
        &blocks::GRAY_CARPET.registry_name.clone(),
        GRAY_CARPET.clone(),
    );
    registry.insert(
        &blocks::LIGHT_GRAY_CARPET.registry_name.clone(),
        LIGHT_GRAY_CARPET.clone(),
    );
    registry.insert(
        &blocks::CYAN_CARPET.registry_name.clone(),
        CYAN_CARPET.clone(),
    );
    registry.insert(
        &blocks::PURPLE_CARPET.registry_name.clone(),
        PURPLE_CARPET.clone(),
    );
    registry.insert(
        &blocks::BLUE_CARPET.registry_name.clone(),
        BLUE_CARPET.clone(),
    );
    registry.insert(
        &blocks::BROWN_CARPET.registry_name.clone(),
        BROWN_CARPET.clone(),
    );
    registry.insert(
        &blocks::GREEN_CARPET.registry_name.clone(),
        GREEN_CARPET.clone(),
    );
    registry.insert(
        &blocks::RED_CARPET.registry_name.clone(),
        RED_CARPET.clone(),
    );
    registry.insert(
        &blocks::BLACK_CARPET.registry_name.clone(),
        BLACK_CARPET.clone(),
    );
    registry.insert(
        &blocks::TERRACOTTA.registry_name.clone(),
        TERRACOTTA.clone(),
    );
    registry.insert(
        &blocks::COAL_BLOCK.registry_name.clone(),
        COAL_BLOCK.clone(),
    );
    registry.insert(
        &blocks::PACKED_ICE.registry_name.clone(),
        PACKED_ICE.clone(),
    );
    registry.insert(
        &blocks::ACACIA_STAIRS.registry_name.clone(),
        ACACIA_STAIRS.clone(),
    );
    registry.insert(
        &blocks::DARK_OAK_STAIRS.registry_name.clone(),
        DARK_OAK_STAIRS.clone(),
    );
    registry.insert(
        &blocks::SLIME_BLOCK.registry_name.clone(),
        SLIME_BLOCK.clone(),
    );
    registry.insert(
        &blocks::GRASS_PATH.registry_name.clone(),
        GRASS_PATH.clone(),
    );
    registry.insert(&blocks::SUNFLOWER.registry_name.clone(), SUNFLOWER.clone());
    registry.insert(&blocks::LILAC.registry_name.clone(), LILAC.clone());
    registry.insert(&blocks::ROSE_BUSH.registry_name.clone(), ROSE_BUSH.clone());
    registry.insert(&blocks::PEONY.registry_name.clone(), PEONY.clone());
    registry.insert(
        &blocks::TALL_GRASS.registry_name.clone(),
        TALL_GRASS.clone(),
    );
    registry.insert(
        &blocks::LARGE_FERN.registry_name.clone(),
        LARGE_FERN.clone(),
    );
    registry.insert(
        &blocks::WHITE_STAINED_GLASS.registry_name.clone(),
        WHITE_STAINED_GLASS.clone(),
    );
    registry.insert(
        &blocks::ORANGE_STAINED_GLASS.registry_name.clone(),
        ORANGE_STAINED_GLASS.clone(),
    );
    registry.insert(
        &blocks::MAGENTA_STAINED_GLASS.registry_name.clone(),
        MAGENTA_STAINED_GLASS.clone(),
    );
    registry.insert(
        &blocks::LIGHT_BLUE_STAINED_GLASS.registry_name.clone(),
        LIGHT_BLUE_STAINED_GLASS.clone(),
    );
    registry.insert(
        &blocks::YELLOW_STAINED_GLASS.registry_name.clone(),
        YELLOW_STAINED_GLASS.clone(),
    );
    registry.insert(
        &blocks::LIME_STAINED_GLASS.registry_name.clone(),
        LIME_STAINED_GLASS.clone(),
    );
    registry.insert(
        &blocks::PINK_STAINED_GLASS.registry_name.clone(),
        PINK_STAINED_GLASS.clone(),
    );
    registry.insert(
        &blocks::GRAY_STAINED_GLASS.registry_name.clone(),
        GRAY_STAINED_GLASS.clone(),
    );
    registry.insert(
        &blocks::LIGHT_GRAY_STAINED_GLASS.registry_name.clone(),
        LIGHT_GRAY_STAINED_GLASS.clone(),
    );
    registry.insert(
        &blocks::CYAN_STAINED_GLASS.registry_name.clone(),
        CYAN_STAINED_GLASS.clone(),
    );
    registry.insert(
        &blocks::PURPLE_STAINED_GLASS.registry_name.clone(),
        PURPLE_STAINED_GLASS.clone(),
    );
    registry.insert(
        &blocks::BLUE_STAINED_GLASS.registry_name.clone(),
        BLUE_STAINED_GLASS.clone(),
    );
    registry.insert(
        &blocks::BROWN_STAINED_GLASS.registry_name.clone(),
        BROWN_STAINED_GLASS.clone(),
    );
    registry.insert(
        &blocks::GREEN_STAINED_GLASS.registry_name.clone(),
        GREEN_STAINED_GLASS.clone(),
    );
    registry.insert(
        &blocks::RED_STAINED_GLASS.registry_name.clone(),
        RED_STAINED_GLASS.clone(),
    );
    registry.insert(
        &blocks::BLACK_STAINED_GLASS.registry_name.clone(),
        BLACK_STAINED_GLASS.clone(),
    );
    registry.insert(
        &blocks::WHITE_STAINED_GLASS_PANE.registry_name.clone(),
        WHITE_STAINED_GLASS_PANE.clone(),
    );
    registry.insert(
        &blocks::ORANGE_STAINED_GLASS_PANE.registry_name.clone(),
        ORANGE_STAINED_GLASS_PANE.clone(),
    );
    registry.insert(
        &blocks::MAGENTA_STAINED_GLASS_PANE.registry_name.clone(),
        MAGENTA_STAINED_GLASS_PANE.clone(),
    );
    registry.insert(
        &blocks::LIGHT_BLUE_STAINED_GLASS_PANE.registry_name.clone(),
        LIGHT_BLUE_STAINED_GLASS_PANE.clone(),
    );
    registry.insert(
        &blocks::YELLOW_STAINED_GLASS_PANE.registry_name.clone(),
        YELLOW_STAINED_GLASS_PANE.clone(),
    );
    registry.insert(
        &blocks::LIME_STAINED_GLASS_PANE.registry_name.clone(),
        LIME_STAINED_GLASS_PANE.clone(),
    );
    registry.insert(
        &blocks::PINK_STAINED_GLASS_PANE.registry_name.clone(),
        PINK_STAINED_GLASS_PANE.clone(),
    );
    registry.insert(
        &blocks::GRAY_STAINED_GLASS_PANE.registry_name.clone(),
        GRAY_STAINED_GLASS_PANE.clone(),
    );
    registry.insert(
        &blocks::LIGHT_GRAY_STAINED_GLASS_PANE.registry_name.clone(),
        LIGHT_GRAY_STAINED_GLASS_PANE.clone(),
    );
    registry.insert(
        &blocks::CYAN_STAINED_GLASS_PANE.registry_name.clone(),
        CYAN_STAINED_GLASS_PANE.clone(),
    );
    registry.insert(
        &blocks::PURPLE_STAINED_GLASS_PANE.registry_name.clone(),
        PURPLE_STAINED_GLASS_PANE.clone(),
    );
    registry.insert(
        &blocks::BLUE_STAINED_GLASS_PANE.registry_name.clone(),
        BLUE_STAINED_GLASS_PANE.clone(),
    );
    registry.insert(
        &blocks::BROWN_STAINED_GLASS_PANE.registry_name.clone(),
        BROWN_STAINED_GLASS_PANE.clone(),
    );
    registry.insert(
        &blocks::GREEN_STAINED_GLASS_PANE.registry_name.clone(),
        GREEN_STAINED_GLASS_PANE.clone(),
    );
    registry.insert(
        &blocks::RED_STAINED_GLASS_PANE.registry_name.clone(),
        RED_STAINED_GLASS_PANE.clone(),
    );
    registry.insert(
        &blocks::BLACK_STAINED_GLASS_PANE.registry_name.clone(),
        BLACK_STAINED_GLASS_PANE.clone(),
    );
    registry.insert(
        &blocks::PRISMARINE.registry_name.clone(),
        PRISMARINE.clone(),
    );
    registry.insert(
        &blocks::PRISMARINE_BRICKS.registry_name.clone(),
        PRISMARINE_BRICKS.clone(),
    );
    registry.insert(
        &blocks::DARK_PRISMARINE.registry_name.clone(),
        DARK_PRISMARINE.clone(),
    );
    registry.insert(
        &blocks::PRISMARINE_STAIRS.registry_name.clone(),
        PRISMARINE_STAIRS.clone(),
    );
    registry.insert(
        &blocks::PRISMARINE_BRICK_STAIRS.registry_name.clone(),
        PRISMARINE_BRICK_STAIRS.clone(),
    );
    registry.insert(
        &blocks::DARK_PRISMARINE_STAIRS.registry_name.clone(),
        DARK_PRISMARINE_STAIRS.clone(),
    );
    registry.insert(
        &blocks::SEA_LANTERN.registry_name.clone(),
        SEA_LANTERN.clone(),
    );
    registry.insert(
        &blocks::RED_SANDSTONE.registry_name.clone(),
        RED_SANDSTONE.clone(),
    );
    registry.insert(
        &blocks::CHISELED_RED_SANDSTONE.registry_name.clone(),
        CHISELED_RED_SANDSTONE.clone(),
    );
    registry.insert(
        &blocks::CUT_RED_SANDSTONE.registry_name.clone(),
        CUT_RED_SANDSTONE.clone(),
    );
    registry.insert(
        &blocks::RED_SANDSTONE_STAIRS.registry_name.clone(),
        RED_SANDSTONE_STAIRS.clone(),
    );
    registry.insert(
        &blocks::REPEATING_COMMAND_BLOCK.registry_name.clone(),
        REPEATING_COMMAND_BLOCK.clone(),
    );
    registry.insert(
        &blocks::CHAIN_COMMAND_BLOCK.registry_name.clone(),
        CHAIN_COMMAND_BLOCK.clone(),
    );
    registry.insert(
        &blocks::MAGMA_BLOCK.registry_name.clone(),
        MAGMA_BLOCK.clone(),
    );
    registry.insert(
        &blocks::NETHER_WART_BLOCK.registry_name.clone(),
        NETHER_WART_BLOCK.clone(),
    );
    registry.insert(
        &blocks::RED_NETHER_BRICKS.registry_name.clone(),
        RED_NETHER_BRICKS.clone(),
    );
    registry.insert(
        &blocks::BONE_BLOCK.registry_name.clone(),
        BONE_BLOCK.clone(),
    );
    registry.insert(
        &blocks::STRUCTURE_VOID.registry_name.clone(),
        STRUCTURE_VOID.clone(),
    );
    registry.insert(&blocks::OBSERVER.registry_name.clone(), OBSERVER.clone());
    registry.insert(
        &blocks::SHULKER_BOX.registry_name.clone(),
        SHULKER_BOX.clone(),
    );
    registry.insert(
        &blocks::WHITE_SHULKER_BOX.registry_name.clone(),
        WHITE_SHULKER_BOX.clone(),
    );
    registry.insert(
        &blocks::ORANGE_SHULKER_BOX.registry_name.clone(),
        ORANGE_SHULKER_BOX.clone(),
    );
    registry.insert(
        &blocks::MAGENTA_SHULKER_BOX.registry_name.clone(),
        MAGENTA_SHULKER_BOX.clone(),
    );
    registry.insert(
        &blocks::LIGHT_BLUE_SHULKER_BOX.registry_name.clone(),
        LIGHT_BLUE_SHULKER_BOX.clone(),
    );
    registry.insert(
        &blocks::YELLOW_SHULKER_BOX.registry_name.clone(),
        YELLOW_SHULKER_BOX.clone(),
    );
    registry.insert(
        &blocks::LIME_SHULKER_BOX.registry_name.clone(),
        LIME_SHULKER_BOX.clone(),
    );
    registry.insert(
        &blocks::PINK_SHULKER_BOX.registry_name.clone(),
        PINK_SHULKER_BOX.clone(),
    );
    registry.insert(
        &blocks::GRAY_SHULKER_BOX.registry_name.clone(),
        GRAY_SHULKER_BOX.clone(),
    );
    registry.insert(
        &blocks::LIGHT_GRAY_SHULKER_BOX.registry_name.clone(),
        LIGHT_GRAY_SHULKER_BOX.clone(),
    );
    registry.insert(
        &blocks::CYAN_SHULKER_BOX.registry_name.clone(),
        CYAN_SHULKER_BOX.clone(),
    );
    registry.insert(
        &blocks::PURPLE_SHULKER_BOX.registry_name.clone(),
        PURPLE_SHULKER_BOX.clone(),
    );
    registry.insert(
        &blocks::BLUE_SHULKER_BOX.registry_name.clone(),
        BLUE_SHULKER_BOX.clone(),
    );
    registry.insert(
        &blocks::BROWN_SHULKER_BOX.registry_name.clone(),
        BROWN_SHULKER_BOX.clone(),
    );
    registry.insert(
        &blocks::GREEN_SHULKER_BOX.registry_name.clone(),
        GREEN_SHULKER_BOX.clone(),
    );
    registry.insert(
        &blocks::RED_SHULKER_BOX.registry_name.clone(),
        RED_SHULKER_BOX.clone(),
    );
    registry.insert(
        &blocks::BLACK_SHULKER_BOX.registry_name.clone(),
        BLACK_SHULKER_BOX.clone(),
    );
    registry.insert(
        &blocks::WHITE_GLAZED_TERRACOTTA.registry_name.clone(),
        WHITE_GLAZED_TERRACOTTA.clone(),
    );
    registry.insert(
        &blocks::ORANGE_GLAZED_TERRACOTTA.registry_name.clone(),
        ORANGE_GLAZED_TERRACOTTA.clone(),
    );
    registry.insert(
        &blocks::MAGENTA_GLAZED_TERRACOTTA.registry_name.clone(),
        MAGENTA_GLAZED_TERRACOTTA.clone(),
    );
    registry.insert(
        &blocks::LIGHT_BLUE_GLAZED_TERRACOTTA.registry_name.clone(),
        LIGHT_BLUE_GLAZED_TERRACOTTA.clone(),
    );
    registry.insert(
        &blocks::YELLOW_GLAZED_TERRACOTTA.registry_name.clone(),
        YELLOW_GLAZED_TERRACOTTA.clone(),
    );
    registry.insert(
        &blocks::LIME_GLAZED_TERRACOTTA.registry_name.clone(),
        LIME_GLAZED_TERRACOTTA.clone(),
    );
    registry.insert(
        &blocks::PINK_GLAZED_TERRACOTTA.registry_name.clone(),
        PINK_GLAZED_TERRACOTTA.clone(),
    );
    registry.insert(
        &blocks::GRAY_GLAZED_TERRACOTTA.registry_name.clone(),
        GRAY_GLAZED_TERRACOTTA.clone(),
    );
    registry.insert(
        &blocks::LIGHT_GRAY_GLAZED_TERRACOTTA.registry_name.clone(),
        LIGHT_GRAY_GLAZED_TERRACOTTA.clone(),
    );
    registry.insert(
        &blocks::CYAN_GLAZED_TERRACOTTA.registry_name.clone(),
        CYAN_GLAZED_TERRACOTTA.clone(),
    );
    registry.insert(
        &blocks::PURPLE_GLAZED_TERRACOTTA.registry_name.clone(),
        PURPLE_GLAZED_TERRACOTTA.clone(),
    );
    registry.insert(
        &blocks::BLUE_GLAZED_TERRACOTTA.registry_name.clone(),
        BLUE_GLAZED_TERRACOTTA.clone(),
    );
    registry.insert(
        &blocks::BROWN_GLAZED_TERRACOTTA.registry_name.clone(),
        BROWN_GLAZED_TERRACOTTA.clone(),
    );
    registry.insert(
        &blocks::GREEN_GLAZED_TERRACOTTA.registry_name.clone(),
        GREEN_GLAZED_TERRACOTTA.clone(),
    );
    registry.insert(
        &blocks::RED_GLAZED_TERRACOTTA.registry_name.clone(),
        RED_GLAZED_TERRACOTTA.clone(),
    );
    registry.insert(
        &blocks::BLACK_GLAZED_TERRACOTTA.registry_name.clone(),
        BLACK_GLAZED_TERRACOTTA.clone(),
    );
    registry.insert(
        &blocks::WHITE_CONCRETE.registry_name.clone(),
        WHITE_CONCRETE.clone(),
    );
    registry.insert(
        &blocks::ORANGE_CONCRETE.registry_name.clone(),
        ORANGE_CONCRETE.clone(),
    );
    registry.insert(
        &blocks::MAGENTA_CONCRETE.registry_name.clone(),
        MAGENTA_CONCRETE.clone(),
    );
    registry.insert(
        &blocks::LIGHT_BLUE_CONCRETE.registry_name.clone(),
        LIGHT_BLUE_CONCRETE.clone(),
    );
    registry.insert(
        &blocks::YELLOW_CONCRETE.registry_name.clone(),
        YELLOW_CONCRETE.clone(),
    );
    registry.insert(
        &blocks::LIME_CONCRETE.registry_name.clone(),
        LIME_CONCRETE.clone(),
    );
    registry.insert(
        &blocks::PINK_CONCRETE.registry_name.clone(),
        PINK_CONCRETE.clone(),
    );
    registry.insert(
        &blocks::GRAY_CONCRETE.registry_name.clone(),
        GRAY_CONCRETE.clone(),
    );
    registry.insert(
        &blocks::LIGHT_GRAY_CONCRETE.registry_name.clone(),
        LIGHT_GRAY_CONCRETE.clone(),
    );
    registry.insert(
        &blocks::CYAN_CONCRETE.registry_name.clone(),
        CYAN_CONCRETE.clone(),
    );
    registry.insert(
        &blocks::PURPLE_CONCRETE.registry_name.clone(),
        PURPLE_CONCRETE.clone(),
    );
    registry.insert(
        &blocks::BLUE_CONCRETE.registry_name.clone(),
        BLUE_CONCRETE.clone(),
    );
    registry.insert(
        &blocks::BROWN_CONCRETE.registry_name.clone(),
        BROWN_CONCRETE.clone(),
    );
    registry.insert(
        &blocks::GREEN_CONCRETE.registry_name.clone(),
        GREEN_CONCRETE.clone(),
    );
    registry.insert(
        &blocks::RED_CONCRETE.registry_name.clone(),
        RED_CONCRETE.clone(),
    );
    registry.insert(
        &blocks::BLACK_CONCRETE.registry_name.clone(),
        BLACK_CONCRETE.clone(),
    );
    registry.insert(
        &blocks::WHITE_CONCRETE_POWDER.registry_name.clone(),
        WHITE_CONCRETE_POWDER.clone(),
    );
    registry.insert(
        &blocks::ORANGE_CONCRETE_POWDER.registry_name.clone(),
        ORANGE_CONCRETE_POWDER.clone(),
    );
    registry.insert(
        &blocks::MAGENTA_CONCRETE_POWDER.registry_name.clone(),
        MAGENTA_CONCRETE_POWDER.clone(),
    );
    registry.insert(
        &blocks::LIGHT_BLUE_CONCRETE_POWDER.registry_name.clone(),
        LIGHT_BLUE_CONCRETE_POWDER.clone(),
    );
    registry.insert(
        &blocks::YELLOW_CONCRETE_POWDER.registry_name.clone(),
        YELLOW_CONCRETE_POWDER.clone(),
    );
    registry.insert(
        &blocks::LIME_CONCRETE_POWDER.registry_name.clone(),
        LIME_CONCRETE_POWDER.clone(),
    );
    registry.insert(
        &blocks::PINK_CONCRETE_POWDER.registry_name.clone(),
        PINK_CONCRETE_POWDER.clone(),
    );
    registry.insert(
        &blocks::GRAY_CONCRETE_POWDER.registry_name.clone(),
        GRAY_CONCRETE_POWDER.clone(),
    );
    registry.insert(
        &blocks::LIGHT_GRAY_CONCRETE_POWDER.registry_name.clone(),
        LIGHT_GRAY_CONCRETE_POWDER.clone(),
    );
    registry.insert(
        &blocks::CYAN_CONCRETE_POWDER.registry_name.clone(),
        CYAN_CONCRETE_POWDER.clone(),
    );
    registry.insert(
        &blocks::PURPLE_CONCRETE_POWDER.registry_name.clone(),
        PURPLE_CONCRETE_POWDER.clone(),
    );
    registry.insert(
        &blocks::BLUE_CONCRETE_POWDER.registry_name.clone(),
        BLUE_CONCRETE_POWDER.clone(),
    );
    registry.insert(
        &blocks::BROWN_CONCRETE_POWDER.registry_name.clone(),
        BROWN_CONCRETE_POWDER.clone(),
    );
    registry.insert(
        &blocks::GREEN_CONCRETE_POWDER.registry_name.clone(),
        GREEN_CONCRETE_POWDER.clone(),
    );
    registry.insert(
        &blocks::RED_CONCRETE_POWDER.registry_name.clone(),
        RED_CONCRETE_POWDER.clone(),
    );
    registry.insert(
        &blocks::BLACK_CONCRETE_POWDER.registry_name.clone(),
        BLACK_CONCRETE_POWDER.clone(),
    );
    registry.insert(
        &blocks::TURTLE_EGG.registry_name.clone(),
        TURTLE_EGG.clone(),
    );
    registry.insert(
        &blocks::DEAD_TUBE_CORAL_BLOCK.registry_name.clone(),
        DEAD_TUBE_CORAL_BLOCK.clone(),
    );
    registry.insert(
        &blocks::DEAD_BRAIN_CORAL_BLOCK.registry_name.clone(),
        DEAD_BRAIN_CORAL_BLOCK.clone(),
    );
    registry.insert(
        &blocks::DEAD_BUBBLE_CORAL_BLOCK.registry_name.clone(),
        DEAD_BUBBLE_CORAL_BLOCK.clone(),
    );
    registry.insert(
        &blocks::DEAD_FIRE_CORAL_BLOCK.registry_name.clone(),
        DEAD_FIRE_CORAL_BLOCK.clone(),
    );
    registry.insert(
        &blocks::DEAD_HORN_CORAL_BLOCK.registry_name.clone(),
        DEAD_HORN_CORAL_BLOCK.clone(),
    );
    registry.insert(
        &blocks::TUBE_CORAL_BLOCK.registry_name.clone(),
        TUBE_CORAL_BLOCK.clone(),
    );
    registry.insert(
        &blocks::BRAIN_CORAL_BLOCK.registry_name.clone(),
        BRAIN_CORAL_BLOCK.clone(),
    );
    registry.insert(
        &blocks::BUBBLE_CORAL_BLOCK.registry_name.clone(),
        BUBBLE_CORAL_BLOCK.clone(),
    );
    registry.insert(
        &blocks::FIRE_CORAL_BLOCK.registry_name.clone(),
        FIRE_CORAL_BLOCK.clone(),
    );
    registry.insert(
        &blocks::HORN_CORAL_BLOCK.registry_name.clone(),
        HORN_CORAL_BLOCK.clone(),
    );
    registry.insert(
        &blocks::TUBE_CORAL.registry_name.clone(),
        TUBE_CORAL.clone(),
    );
    registry.insert(
        &blocks::BRAIN_CORAL.registry_name.clone(),
        BRAIN_CORAL.clone(),
    );
    registry.insert(
        &blocks::BUBBLE_CORAL.registry_name.clone(),
        BUBBLE_CORAL.clone(),
    );
    registry.insert(
        &blocks::FIRE_CORAL.registry_name.clone(),
        FIRE_CORAL.clone(),
    );
    registry.insert(
        &blocks::HORN_CORAL.registry_name.clone(),
        HORN_CORAL.clone(),
    );
    registry.insert(
        &blocks::DEAD_BRAIN_CORAL.registry_name.clone(),
        DEAD_BRAIN_CORAL.clone(),
    );
    registry.insert(
        &blocks::DEAD_BUBBLE_CORAL.registry_name.clone(),
        DEAD_BUBBLE_CORAL.clone(),
    );
    registry.insert(
        &blocks::DEAD_FIRE_CORAL.registry_name.clone(),
        DEAD_FIRE_CORAL.clone(),
    );
    registry.insert(
        &blocks::DEAD_HORN_CORAL.registry_name.clone(),
        DEAD_HORN_CORAL.clone(),
    );
    registry.insert(
        &blocks::DEAD_TUBE_CORAL.registry_name.clone(),
        DEAD_TUBE_CORAL.clone(),
    );
    registry.insert(
        &blocks::TUBE_CORAL_FAN.registry_name.clone(),
        TUBE_CORAL_FAN.clone(),
    );
    registry.insert(
        &blocks::BRAIN_CORAL_FAN.registry_name.clone(),
        BRAIN_CORAL_FAN.clone(),
    );
    registry.insert(
        &blocks::BUBBLE_CORAL_FAN.registry_name.clone(),
        BUBBLE_CORAL_FAN.clone(),
    );
    registry.insert(
        &blocks::FIRE_CORAL_FAN.registry_name.clone(),
        FIRE_CORAL_FAN.clone(),
    );
    registry.insert(
        &blocks::HORN_CORAL_FAN.registry_name.clone(),
        HORN_CORAL_FAN.clone(),
    );
    registry.insert(
        &blocks::DEAD_TUBE_CORAL_FAN.registry_name.clone(),
        DEAD_TUBE_CORAL_FAN.clone(),
    );
    registry.insert(
        &blocks::DEAD_BRAIN_CORAL_FAN.registry_name.clone(),
        DEAD_BRAIN_CORAL_FAN.clone(),
    );
    registry.insert(
        &blocks::DEAD_BUBBLE_CORAL_FAN.registry_name.clone(),
        DEAD_BUBBLE_CORAL_FAN.clone(),
    );
    registry.insert(
        &blocks::DEAD_FIRE_CORAL_FAN.registry_name.clone(),
        DEAD_FIRE_CORAL_FAN.clone(),
    );
    registry.insert(
        &blocks::DEAD_HORN_CORAL_FAN.registry_name.clone(),
        DEAD_HORN_CORAL_FAN.clone(),
    );
    registry.insert(&blocks::BLUE_ICE.registry_name.clone(), BLUE_ICE.clone());
    registry.insert(&blocks::CONDUIT.registry_name.clone(), CONDUIT.clone());
    registry.insert(
        &blocks::POLISHED_GRANITE_STAIRS.registry_name.clone(),
        POLISHED_GRANITE_STAIRS.clone(),
    );
    registry.insert(
        &blocks::SMOOTH_RED_SANDSTONE_STAIRS.registry_name.clone(),
        SMOOTH_RED_SANDSTONE_STAIRS.clone(),
    );
    registry.insert(
        &blocks::MOSSY_STONE_BRICK_STAIRS.registry_name.clone(),
        MOSSY_STONE_BRICK_STAIRS.clone(),
    );
    registry.insert(
        &blocks::POLISHED_DIORITE_STAIRS.registry_name.clone(),
        POLISHED_DIORITE_STAIRS.clone(),
    );
    registry.insert(
        &blocks::MOSSY_COBBLESTONE_STAIRS.registry_name.clone(),
        MOSSY_COBBLESTONE_STAIRS.clone(),
    );
    registry.insert(
        &blocks::END_STONE_BRICK_STAIRS.registry_name.clone(),
        END_STONE_BRICK_STAIRS.clone(),
    );
    registry.insert(
        &blocks::STONE_STAIRS.registry_name.clone(),
        STONE_STAIRS.clone(),
    );
    registry.insert(
        &blocks::SMOOTH_SANDSTONE_STAIRS.registry_name.clone(),
        SMOOTH_SANDSTONE_STAIRS.clone(),
    );
    registry.insert(
        &blocks::SMOOTH_QUARTZ_STAIRS.registry_name.clone(),
        SMOOTH_QUARTZ_STAIRS.clone(),
    );
    registry.insert(
        &blocks::GRANITE_STAIRS.registry_name.clone(),
        GRANITE_STAIRS.clone(),
    );
    registry.insert(
        &blocks::ANDESITE_STAIRS.registry_name.clone(),
        ANDESITE_STAIRS.clone(),
    );
    registry.insert(
        &blocks::RED_NETHER_BRICK_STAIRS.registry_name.clone(),
        RED_NETHER_BRICK_STAIRS.clone(),
    );
    registry.insert(
        &blocks::POLISHED_ANDESITE_STAIRS.registry_name.clone(),
        POLISHED_ANDESITE_STAIRS.clone(),
    );
    registry.insert(
        &blocks::DIORITE_STAIRS.registry_name.clone(),
        DIORITE_STAIRS.clone(),
    );
    registry.insert(
        &blocks::POLISHED_GRANITE_SLAB.registry_name.clone(),
        POLISHED_GRANITE_SLAB.clone(),
    );
    registry.insert(
        &blocks::SMOOTH_RED_SANDSTONE_SLAB.registry_name.clone(),
        SMOOTH_RED_SANDSTONE_SLAB.clone(),
    );
    registry.insert(
        &blocks::MOSSY_STONE_BRICK_SLAB.registry_name.clone(),
        MOSSY_STONE_BRICK_SLAB.clone(),
    );
    registry.insert(
        &blocks::POLISHED_DIORITE_SLAB.registry_name.clone(),
        POLISHED_DIORITE_SLAB.clone(),
    );
    registry.insert(
        &blocks::MOSSY_COBBLESTONE_SLAB.registry_name.clone(),
        MOSSY_COBBLESTONE_SLAB.clone(),
    );
    registry.insert(
        &blocks::END_STONE_BRICK_SLAB.registry_name.clone(),
        END_STONE_BRICK_SLAB.clone(),
    );
    registry.insert(
        &blocks::SMOOTH_SANDSTONE_SLAB.registry_name.clone(),
        SMOOTH_SANDSTONE_SLAB.clone(),
    );
    registry.insert(
        &blocks::SMOOTH_QUARTZ_SLAB.registry_name.clone(),
        SMOOTH_QUARTZ_SLAB.clone(),
    );
    registry.insert(
        &blocks::GRANITE_SLAB.registry_name.clone(),
        GRANITE_SLAB.clone(),
    );
    registry.insert(
        &blocks::ANDESITE_SLAB.registry_name.clone(),
        ANDESITE_SLAB.clone(),
    );
    registry.insert(
        &blocks::RED_NETHER_BRICK_SLAB.registry_name.clone(),
        RED_NETHER_BRICK_SLAB.clone(),
    );
    registry.insert(
        &blocks::POLISHED_ANDESITE_SLAB.registry_name.clone(),
        POLISHED_ANDESITE_SLAB.clone(),
    );
    registry.insert(
        &blocks::DIORITE_SLAB.registry_name.clone(),
        DIORITE_SLAB.clone(),
    );
    registry.insert(
        &blocks::SCAFFOLDING.registry_name.clone(),
        SCAFFOLDING.clone(),
    );
    registry.insert(&blocks::IRON_DOOR.registry_name.clone(), IRON_DOOR.clone());
    registry.insert(&blocks::OAK_DOOR.registry_name.clone(), OAK_DOOR.clone());
    registry.insert(
        &blocks::SPRUCE_DOOR.registry_name.clone(),
        SPRUCE_DOOR.clone(),
    );
    registry.insert(
        &blocks::BIRCH_DOOR.registry_name.clone(),
        BIRCH_DOOR.clone(),
    );
    registry.insert(
        &blocks::JUNGLE_DOOR.registry_name.clone(),
        JUNGLE_DOOR.clone(),
    );
    registry.insert(
        &blocks::ACACIA_DOOR.registry_name.clone(),
        ACACIA_DOOR.clone(),
    );
    registry.insert(
        &blocks::DARK_OAK_DOOR.registry_name.clone(),
        DARK_OAK_DOOR.clone(),
    );
    registry.insert(&blocks::REPEATER.registry_name.clone(), REPEATER.clone());
    registry.insert(
        &blocks::COMPARATOR.registry_name.clone(),
        COMPARATOR.clone(),
    );
    registry.insert(
        &blocks::STRUCTURE_BLOCK.registry_name.clone(),
        STRUCTURE_BLOCK.clone(),
    );
    registry.insert(&blocks::JIGSAW.registry_name.clone(), JIGSAW.clone());
    registry.insert(&blocks::COMPOSTER.registry_name.clone(), COMPOSTER.clone());
    registry.insert("minecraft:turtle_helmet", TURTLE_HELMET.clone());
    registry.insert("minecraft:scute", SCUTE.clone());
    registry.insert("minecraft:iron_shovel", IRON_SHOVEL.clone());
    registry.insert("minecraft:iron_pickaxe", IRON_PICKAXE.clone());
    registry.insert("minecraft:iron_axe", IRON_AXE.clone());
    registry.insert("minecraft:flint_and_steel", FLINT_AND_STEEL.clone());
    registry.insert("minecraft:apple", APPLE.clone());
    registry.insert("minecraft:bow", BOW.clone());
    registry.insert("minecraft:arrow", ARROW.clone());
    registry.insert("minecraft:coal", COAL.clone());
    registry.insert("minecraft:charcoal", CHARCOAL.clone());
    registry.insert("minecraft:diamond", DIAMOND.clone());
    registry.insert("minecraft:iron_ingot", IRON_INGOT.clone());
    registry.insert("minecraft:gold_ingot", GOLD_INGOT.clone());
    registry.insert("minecraft:iron_sword", IRON_SWORD.clone());
    registry.insert("minecraft:wooden_sword", WOODEN_SWORD.clone());
    registry.insert("minecraft:wooden_shovel", WOODEN_SHOVEL.clone());
    registry.insert("minecraft:wooden_pickaxe", WOODEN_PICKAXE.clone());
    registry.insert("minecraft:wooden_axe", WOODEN_AXE.clone());
    registry.insert("minecraft:stone_sword", STONE_SWORD.clone());
    registry.insert("minecraft:stone_shovel", STONE_SHOVEL.clone());
    registry.insert("minecraft:stone_pickaxe", STONE_PICKAXE.clone());
    registry.insert("minecraft:stone_axe", STONE_AXE.clone());
    registry.insert("minecraft:diamond_sword", DIAMOND_SWORD.clone());
    registry.insert("minecraft:diamond_shovel", DIAMOND_SHOVEL.clone());
    registry.insert("minecraft:diamond_pickaxe", DIAMOND_PICKAXE.clone());
    registry.insert("minecraft:diamond_axe", DIAMOND_AXE.clone());
    registry.insert("minecraft:stick", STICK.clone());
    registry.insert("minecraft:bowl", BOWL.clone());
    registry.insert("minecraft:mushroom_stew", MUSHROOM_STEW.clone());
    registry.insert("minecraft:golden_sword", GOLDEN_SWORD.clone());
    registry.insert("minecraft:golden_shovel", GOLDEN_SHOVEL.clone());
    registry.insert("minecraft:golden_pickaxe", GOLDEN_PICKAXE.clone());
    registry.insert("minecraft:golden_axe", GOLDEN_AXE.clone());
    registry.insert("minecraft:string", STRING.clone());
    registry.insert("minecraft:feather", FEATHER.clone());
    registry.insert("minecraft:gunpowder", GUNPOWDER.clone());
    registry.insert("minecraft:wooden_hoe", WOODEN_HOE.clone());
    registry.insert("minecraft:stone_hoe", STONE_HOE.clone());
    registry.insert("minecraft:iron_hoe", IRON_HOE.clone());
    registry.insert("minecraft:diamond_hoe", DIAMOND_HOE.clone());
    registry.insert("minecraft:golden_hoe", GOLDEN_HOE.clone());
    registry.insert("minecraft:wheat_seeds", WHEAT_SEEDS.clone());
    registry.insert("minecraft:wheat", WHEAT.clone());
    registry.insert("minecraft:bread", BREAD.clone());
    registry.insert("minecraft:leather_helmet", LEATHER_HELMET.clone());
    registry.insert("minecraft:leather_chestplate", LEATHER_CHESTPLATE.clone());
    registry.insert("minecraft:leather_leggings", LEATHER_LEGGINGS.clone());
    registry.insert("minecraft:leather_boots", LEATHER_BOOTS.clone());
    registry.insert("minecraft:chainmail_helmet", CHAINMAIL_HELMET.clone());
    registry.insert(
        "minecraft:chainmail_chestplate",
        CHAINMAIL_CHESTPLATE.clone(),
    );
    registry.insert("minecraft:chainmail_leggings", CHAINMAIL_LEGGINGS.clone());
    registry.insert("minecraft:chainmail_boots", CHAINMAIL_BOOTS.clone());
    registry.insert("minecraft:iron_helmet", IRON_HELMET.clone());
    registry.insert("minecraft:iron_chestplate", IRON_CHESTPLATE.clone());
    registry.insert("minecraft:iron_leggings", IRON_LEGGINGS.clone());
    registry.insert("minecraft:iron_boots", IRON_BOOTS.clone());
    registry.insert("minecraft:diamond_helmet", DIAMOND_HELMET.clone());
    registry.insert("minecraft:diamond_chestplate", DIAMOND_CHESTPLATE.clone());
    registry.insert("minecraft:diamond_leggings", DIAMOND_LEGGINGS.clone());
    registry.insert("minecraft:diamond_boots", DIAMOND_BOOTS.clone());
    registry.insert("minecraft:golden_helmet", GOLDEN_HELMET.clone());
    registry.insert("minecraft:golden_chestplate", GOLDEN_CHESTPLATE.clone());
    registry.insert("minecraft:golden_leggings", GOLDEN_LEGGINGS.clone());
    registry.insert("minecraft:golden_boots", GOLDEN_BOOTS.clone());
    registry.insert("minecraft:flint", FLINT.clone());
    registry.insert("minecraft:porkchop", PORKCHOP.clone());
    registry.insert("minecraft:cooked_porkchop", COOKED_PORKCHOP.clone());
    registry.insert("minecraft:painting", PAINTING.clone());
    registry.insert("minecraft:golden_apple", GOLDEN_APPLE.clone());
    registry.insert(
        "minecraft:enchanted_golden_apple",
        ENCHANTED_GOLDEN_APPLE.clone(),
    );
    registry.insert("minecraft:oak_sign", OAK_SIGN.clone());
    registry.insert("minecraft:spruce_sign", SPRUCE_SIGN.clone());
    registry.insert("minecraft:birch_sign", BIRCH_SIGN.clone());
    registry.insert("minecraft:jungle_sign", JUNGLE_SIGN.clone());
    registry.insert("minecraft:acacia_sign", ACACIA_SIGN.clone());
    registry.insert("minecraft:dark_oak_sign", DARK_OAK_SIGN.clone());
    registry.insert("minecraft:bucket", BUCKET.clone());
    registry.insert("minecraft:water_bucket", WATER_BUCKET.clone());
    registry.insert("minecraft:lava_bucket", LAVA_BUCKET.clone());
    registry.insert("minecraft:minecart", MINECART.clone());
    registry.insert("minecraft:saddle", SADDLE.clone());
    registry.insert("minecraft:redstone", REDSTONE.clone());
    registry.insert("minecraft:snowball", SNOWBALL.clone());
    registry.insert("minecraft:oak_boat", OAK_BOAT.clone());
    registry.insert("minecraft:leather", LEATHER.clone());
    registry.insert("minecraft:milk_bucket", MILK_BUCKET.clone());
    registry.insert("minecraft:pufferfish_bucket", PUFFERFISH_BUCKET.clone());
    registry.insert("minecraft:salmon_bucket", SALMON_BUCKET.clone());
    registry.insert("minecraft:cod_bucket", COD_BUCKET.clone());
    registry.insert(
        "minecraft:tropical_fish_bucket",
        TROPICAL_FISH_BUCKET.clone(),
    );
    registry.insert("minecraft:brick", BRICK.clone());
    registry.insert("minecraft:clay_ball", CLAY_BALL.clone());
    registry.insert(
        &blocks::SUGAR_CANE.registry_name.clone(),
        SUGAR_CANE.clone(),
    );
    registry.insert(&blocks::KELP.registry_name.clone(), KELP.clone());
    registry.insert(
        &blocks::DRIED_KELP_BLOCK.registry_name.clone(),
        DRIED_KELP_BLOCK.clone(),
    );
    registry.insert(&blocks::BAMBOO.registry_name.clone(), BAMBOO.clone());
    registry.insert("minecraft:paper", PAPER.clone());
    registry.insert("minecraft:book", BOOK.clone());
    registry.insert("minecraft:slime_ball", SLIME_BALL.clone());
    registry.insert("minecraft:chest_minecart", CHEST_MINECART.clone());
    registry.insert("minecraft:furnace_minecart", FURNACE_MINECART.clone());
    registry.insert("minecraft:egg", EGG.clone());
    registry.insert("minecraft:compass", COMPASS.clone());
    registry.insert("minecraft:fishing_rod", FISHING_ROD.clone());
    registry.insert("minecraft:clock", CLOCK.clone());
    registry.insert("minecraft:glowstone_dust", GLOWSTONE_DUST.clone());
    registry.insert("minecraft:cod", COD.clone());
    registry.insert("minecraft:salmon", SALMON.clone());
    registry.insert("minecraft:tropical_fish", TROPICAL_FISH.clone());
    registry.insert("minecraft:pufferfish", PUFFERFISH.clone());
    registry.insert("minecraft:cooked_cod", COOKED_COD.clone());
    registry.insert("minecraft:cooked_salmon", COOKED_SALMON.clone());
    registry.insert("minecraft:ink_sac", INK_SAC.clone());
    registry.insert("minecraft:red_dye", RED_DYE.clone());
    registry.insert("minecraft:green_dye", GREEN_DYE.clone());
    registry.insert("minecraft:cocoa_beans", COCOA_BEANS.clone());
    registry.insert("minecraft:lapis_lazuli", LAPIS_LAZULI.clone());
    registry.insert("minecraft:purple_dye", PURPLE_DYE.clone());
    registry.insert("minecraft:cyan_dye", CYAN_DYE.clone());
    registry.insert("minecraft:light_gray_dye", LIGHT_GRAY_DYE.clone());
    registry.insert("minecraft:gray_dye", GRAY_DYE.clone());
    registry.insert("minecraft:pink_dye", PINK_DYE.clone());
    registry.insert("minecraft:lime_dye", LIME_DYE.clone());
    registry.insert("minecraft:yellow_dye", YELLOW_DYE.clone());
    registry.insert("minecraft:light_blue_dye", LIGHT_BLUE_DYE.clone());
    registry.insert("minecraft:magenta_dye", MAGENTA_DYE.clone());
    registry.insert("minecraft:orange_dye", ORANGE_DYE.clone());
    registry.insert("minecraft:bone_meal", BONE_MEAL.clone());
    registry.insert("minecraft:blue_dye", BLUE_DYE.clone());
    registry.insert("minecraft:brown_dye", BROWN_DYE.clone());
    registry.insert("minecraft:black_dye", BLACK_DYE.clone());
    registry.insert("minecraft:white_dye", WHITE_DYE.clone());
    registry.insert("minecraft:bone", BONE.clone());
    registry.insert("minecraft:sugar", SUGAR.clone());
    registry.insert(&blocks::CAKE.registry_name.clone(), CAKE.clone());
    registry.insert(&blocks::WHITE_BED.registry_name.clone(), WHITE_BED.clone());
    registry.insert(
        &blocks::ORANGE_BED.registry_name.clone(),
        ORANGE_BED.clone(),
    );
    registry.insert(
        &blocks::MAGENTA_BED.registry_name.clone(),
        MAGENTA_BED.clone(),
    );
    registry.insert(
        &blocks::LIGHT_BLUE_BED.registry_name.clone(),
        LIGHT_BLUE_BED.clone(),
    );
    registry.insert(
        &blocks::YELLOW_BED.registry_name.clone(),
        YELLOW_BED.clone(),
    );
    registry.insert(&blocks::LIME_BED.registry_name.clone(), LIME_BED.clone());
    registry.insert(&blocks::PINK_BED.registry_name.clone(), PINK_BED.clone());
    registry.insert(&blocks::GRAY_BED.registry_name.clone(), GRAY_BED.clone());
    registry.insert(
        &blocks::LIGHT_GRAY_BED.registry_name.clone(),
        LIGHT_GRAY_BED.clone(),
    );
    registry.insert(&blocks::CYAN_BED.registry_name.clone(), CYAN_BED.clone());
    registry.insert(
        &blocks::PURPLE_BED.registry_name.clone(),
        PURPLE_BED.clone(),
    );
    registry.insert(&blocks::BLUE_BED.registry_name.clone(), BLUE_BED.clone());
    registry.insert(&blocks::BROWN_BED.registry_name.clone(), BROWN_BED.clone());
    registry.insert(&blocks::GREEN_BED.registry_name.clone(), GREEN_BED.clone());
    registry.insert(&blocks::RED_BED.registry_name.clone(), RED_BED.clone());
    registry.insert(&blocks::BLACK_BED.registry_name.clone(), BLACK_BED.clone());
    registry.insert("minecraft:cookie", COOKIE.clone());
    registry.insert("minecraft:filled_map", FILLED_MAP.clone());
    registry.insert("minecraft:shears", SHEARS.clone());
    registry.insert("minecraft:melon_slice", MELON_SLICE.clone());
    registry.insert("minecraft:dried_kelp", DRIED_KELP.clone());
    registry.insert("minecraft:pumpkin_seeds", PUMPKIN_SEEDS.clone());
    registry.insert("minecraft:melon_seeds", MELON_SEEDS.clone());
    registry.insert("minecraft:beef", BEEF.clone());
    registry.insert("minecraft:cooked_beef", COOKED_BEEF.clone());
    registry.insert("minecraft:chicken", CHICKEN.clone());
    registry.insert("minecraft:cooked_chicken", COOKED_CHICKEN.clone());
    registry.insert("minecraft:rotten_flesh", ROTTEN_FLESH.clone());
    registry.insert("minecraft:ender_pearl", ENDER_PEARL.clone());
    registry.insert("minecraft:blaze_rod", BLAZE_ROD.clone());
    registry.insert("minecraft:ghast_tear", GHAST_TEAR.clone());
    registry.insert("minecraft:gold_nugget", GOLD_NUGGET.clone());
    registry.insert("minecraft:nether_wart", NETHER_WART.clone());
    registry.insert("minecraft:potion", POTION.clone());
    registry.insert("minecraft:glass_bottle", GLASS_BOTTLE.clone());
    registry.insert("minecraft:spider_eye", SPIDER_EYE.clone());
    registry.insert(
        "minecraft:fermented_spider_eye",
        FERMENTED_SPIDER_EYE.clone(),
    );
    registry.insert("minecraft:blaze_powder", BLAZE_POWDER.clone());
    registry.insert("minecraft:magma_cream", MAGMA_CREAM.clone());
    registry.insert(
        &blocks::BREWING_STAND.registry_name.clone(),
        BREWING_STAND.clone(),
    );
    registry.insert(&blocks::CAULDRON.registry_name.clone(), CAULDRON.clone());
    registry.insert("minecraft:ender_eye", ENDER_EYE.clone());
    registry.insert(
        "minecraft:glistering_melon_slice",
        GLISTERING_MELON_SLICE.clone(),
    );
    registry.insert("minecraft:bat_spawn_egg", BAT_SPAWN_EGG.clone());
    registry.insert("minecraft:bee_spawn_egg", BEE_SPAWN_EGG.clone());
    registry.insert("minecraft:blaze_spawn_egg", BLAZE_SPAWN_EGG.clone());
    registry.insert("minecraft:cat_spawn_egg", CAT_SPAWN_EGG.clone());
    registry.insert(
        "minecraft:cave_spider_spawn_egg",
        CAVE_SPIDER_SPAWN_EGG.clone(),
    );
    registry.insert("minecraft:chicken_spawn_egg", CHICKEN_SPAWN_EGG.clone());
    registry.insert("minecraft:cod_spawn_egg", COD_SPAWN_EGG.clone());
    registry.insert("minecraft:cow_spawn_egg", COW_SPAWN_EGG.clone());
    registry.insert("minecraft:creeper_spawn_egg", CREEPER_SPAWN_EGG.clone());
    registry.insert("minecraft:dolphin_spawn_egg", DOLPHIN_SPAWN_EGG.clone());
    registry.insert("minecraft:donkey_spawn_egg", DONKEY_SPAWN_EGG.clone());
    registry.insert("minecraft:drowned_spawn_egg", DROWNED_SPAWN_EGG.clone());
    registry.insert(
        "minecraft:elder_guardian_spawn_egg",
        ELDER_GUARDIAN_SPAWN_EGG.clone(),
    );
    registry.insert("minecraft:enderman_spawn_egg", ENDERMAN_SPAWN_EGG.clone());
    registry.insert("minecraft:endermite_spawn_egg", ENDERMITE_SPAWN_EGG.clone());
    registry.insert("minecraft:evoker_spawn_egg", EVOKER_SPAWN_EGG.clone());
    registry.insert("minecraft:fox_spawn_egg", FOX_SPAWN_EGG.clone());
    registry.insert("minecraft:ghast_spawn_egg", GHAST_SPAWN_EGG.clone());
    registry.insert("minecraft:guardian_spawn_egg", GUARDIAN_SPAWN_EGG.clone());
    registry.insert("minecraft:horse_spawn_egg", HORSE_SPAWN_EGG.clone());
    registry.insert("minecraft:husk_spawn_egg", HUSK_SPAWN_EGG.clone());
    registry.insert("minecraft:llama_spawn_egg", LLAMA_SPAWN_EGG.clone());
    registry.insert(
        "minecraft:magma_cube_spawn_egg",
        MAGMA_CUBE_SPAWN_EGG.clone(),
    );
    registry.insert("minecraft:mooshroom_spawn_egg", MOOSHROOM_SPAWN_EGG.clone());
    registry.insert("minecraft:mule_spawn_egg", MULE_SPAWN_EGG.clone());
    registry.insert("minecraft:ocelot_spawn_egg", OCELOT_SPAWN_EGG.clone());
    registry.insert("minecraft:panda_spawn_egg", PANDA_SPAWN_EGG.clone());
    registry.insert("minecraft:parrot_spawn_egg", PARROT_SPAWN_EGG.clone());
    registry.insert("minecraft:phantom_spawn_egg", PHANTOM_SPAWN_EGG.clone());
    registry.insert("minecraft:pig_spawn_egg", PIG_SPAWN_EGG.clone());
    registry.insert("minecraft:pillager_spawn_egg", PILLAGER_SPAWN_EGG.clone());
    registry.insert(
        "minecraft:polar_bear_spawn_egg",
        POLAR_BEAR_SPAWN_EGG.clone(),
    );
    registry.insert(
        "minecraft:pufferfish_spawn_egg",
        PUFFERFISH_SPAWN_EGG.clone(),
    );
    registry.insert("minecraft:rabbit_spawn_egg", RABBIT_SPAWN_EGG.clone());
    registry.insert("minecraft:ravager_spawn_egg", RAVAGER_SPAWN_EGG.clone());
    registry.insert("minecraft:salmon_spawn_egg", SALMON_SPAWN_EGG.clone());
    registry.insert("minecraft:sheep_spawn_egg", SHEEP_SPAWN_EGG.clone());
    registry.insert("minecraft:shulker_spawn_egg", SHULKER_SPAWN_EGG.clone());
    registry.insert(
        "minecraft:silverfish_spawn_egg",
        SILVERFISH_SPAWN_EGG.clone(),
    );
    registry.insert("minecraft:skeleton_spawn_egg", SKELETON_SPAWN_EGG.clone());
    registry.insert(
        "minecraft:skeleton_horse_spawn_egg",
        SKELETON_HORSE_SPAWN_EGG.clone(),
    );
    registry.insert("minecraft:slime_spawn_egg", SLIME_SPAWN_EGG.clone());
    registry.insert("minecraft:spider_spawn_egg", SPIDER_SPAWN_EGG.clone());
    registry.insert("minecraft:squid_spawn_egg", SQUID_SPAWN_EGG.clone());
    registry.insert("minecraft:stray_spawn_egg", STRAY_SPAWN_EGG.clone());
    registry.insert(
        "minecraft:trader_llama_spawn_egg",
        TRADER_LLAMA_SPAWN_EGG.clone(),
    );
    registry.insert(
        "minecraft:tropical_fish_spawn_egg",
        TROPICAL_FISH_SPAWN_EGG.clone(),
    );
    registry.insert("minecraft:turtle_spawn_egg", TURTLE_SPAWN_EGG.clone());
    registry.insert("minecraft:vex_spawn_egg", VEX_SPAWN_EGG.clone());
    registry.insert("minecraft:villager_spawn_egg", VILLAGER_SPAWN_EGG.clone());
    registry.insert(
        "minecraft:vindicator_spawn_egg",
        VINDICATOR_SPAWN_EGG.clone(),
    );
    registry.insert(
        "minecraft:wandering_trader_spawn_egg",
        WANDERING_TRADER_SPAWN_EGG.clone(),
    );
    registry.insert("minecraft:witch_spawn_egg", WITCH_SPAWN_EGG.clone());
    registry.insert(
        "minecraft:wither_skeleton_spawn_egg",
        WITHER_SKELETON_SPAWN_EGG.clone(),
    );
    registry.insert("minecraft:wolf_spawn_egg", WOLF_SPAWN_EGG.clone());
    registry.insert("minecraft:zombie_spawn_egg", ZOMBIE_SPAWN_EGG.clone());
    registry.insert(
        "minecraft:zombie_horse_spawn_egg",
        ZOMBIE_HORSE_SPAWN_EGG.clone(),
    );
    registry.insert(
        "minecraft:zombie_pigman_spawn_egg",
        ZOMBIE_PIGMAN_SPAWN_EGG.clone(),
    );
    registry.insert(
        "minecraft:zombie_villager_spawn_egg",
        ZOMBIE_VILLAGER_SPAWN_EGG.clone(),
    );
    registry.insert("minecraft:experience_bottle", EXPERIENCE_BOTTLE.clone());
    registry.insert("minecraft:fire_charge", FIRE_CHARGE.clone());
    registry.insert("minecraft:writable_book", WRITABLE_BOOK.clone());
    registry.insert("minecraft:written_book", WRITTEN_BOOK.clone());
    registry.insert("minecraft:emerald", EMERALD.clone());
    registry.insert("minecraft:item_frame", ITEM_FRAME.clone());
    registry.insert(
        &blocks::FLOWER_POT.registry_name.clone(),
        FLOWER_POT.clone(),
    );
    registry.insert("minecraft:carrot", CARROT.clone());
    registry.insert("minecraft:potato", POTATO.clone());
    registry.insert("minecraft:baked_potato", BAKED_POTATO.clone());
    registry.insert("minecraft:poisonous_potato", POISONOUS_POTATO.clone());
    registry.insert("minecraft:map", MAP.clone());
    registry.insert("minecraft:golden_carrot", GOLDEN_CARROT.clone());
    registry.insert(
        &blocks::SKELETON_SKULL.registry_name.clone(),
        SKELETON_SKULL.clone(),
    );
    registry.insert(
        &blocks::WITHER_SKELETON_SKULL.registry_name.clone(),
        WITHER_SKELETON_SKULL.clone(),
    );
    registry.insert(
        &blocks::PLAYER_HEAD.registry_name.clone(),
        PLAYER_HEAD.clone(),
    );
    registry.insert(
        &blocks::ZOMBIE_HEAD.registry_name.clone(),
        ZOMBIE_HEAD.clone(),
    );
    registry.insert(
        &blocks::CREEPER_HEAD.registry_name.clone(),
        CREEPER_HEAD.clone(),
    );
    registry.insert(
        &blocks::DRAGON_HEAD.registry_name.clone(),
        DRAGON_HEAD.clone(),
    );
    registry.insert("minecraft:carrot_on_a_stick", CARROT_ON_A_STICK.clone());
    registry.insert("minecraft:nether_star", NETHER_STAR.clone());
    registry.insert("minecraft:pumpkin_pie", PUMPKIN_PIE.clone());
    registry.insert("minecraft:firework_rocket", FIREWORK_ROCKET.clone());
    registry.insert("minecraft:firework_star", FIREWORK_STAR.clone());
    registry.insert("minecraft:enchanted_book", ENCHANTED_BOOK.clone());
    registry.insert("minecraft:nether_brick", NETHER_BRICK.clone());
    registry.insert("minecraft:quartz", QUARTZ.clone());
    registry.insert("minecraft:tnt_minecart", TNT_MINECART.clone());
    registry.insert("minecraft:hopper_minecart", HOPPER_MINECART.clone());
    registry.insert("minecraft:prismarine_shard", PRISMARINE_SHARD.clone());
    registry.insert("minecraft:prismarine_crystals", PRISMARINE_CRYSTALS.clone());
    registry.insert("minecraft:rabbit", RABBIT.clone());
    registry.insert("minecraft:cooked_rabbit", COOKED_RABBIT.clone());
    registry.insert("minecraft:rabbit_stew", RABBIT_STEW.clone());
    registry.insert("minecraft:rabbit_foot", RABBIT_FOOT.clone());
    registry.insert("minecraft:rabbit_hide", RABBIT_HIDE.clone());
    registry.insert("minecraft:armor_stand", ARMOR_STAND.clone());
    registry.insert("minecraft:iron_horse_armor", IRON_HORSE_ARMOR.clone());
    registry.insert("minecraft:golden_horse_armor", GOLDEN_HORSE_ARMOR.clone());
    registry.insert("minecraft:diamond_horse_armor", DIAMOND_HORSE_ARMOR.clone());
    registry.insert("minecraft:leather_horse_armor", LEATHER_HORSE_ARMOR.clone());
    registry.insert("minecraft:lead", LEAD.clone());
    registry.insert("minecraft:name_tag", NAME_TAG.clone());
    registry.insert(
        "minecraft:command_block_minecart",
        COMMAND_BLOCK_MINECART.clone(),
    );
    registry.insert("minecraft:mutton", MUTTON.clone());
    registry.insert("minecraft:cooked_mutton", COOKED_MUTTON.clone());
    registry.insert("minecraft:white_banner", WHITE_BANNER.clone());
    registry.insert("minecraft:orange_banner", ORANGE_BANNER.clone());
    registry.insert("minecraft:magenta_banner", MAGENTA_BANNER.clone());
    registry.insert("minecraft:light_blue_banner", LIGHT_BLUE_BANNER.clone());
    registry.insert("minecraft:yellow_banner", YELLOW_BANNER.clone());
    registry.insert("minecraft:lime_banner", LIME_BANNER.clone());
    registry.insert("minecraft:pink_banner", PINK_BANNER.clone());
    registry.insert("minecraft:gray_banner", GRAY_BANNER.clone());
    registry.insert("minecraft:light_gray_banner", LIGHT_GRAY_BANNER.clone());
    registry.insert("minecraft:cyan_banner", CYAN_BANNER.clone());
    registry.insert("minecraft:purple_banner", PURPLE_BANNER.clone());
    registry.insert("minecraft:blue_banner", BLUE_BANNER.clone());
    registry.insert("minecraft:brown_banner", BROWN_BANNER.clone());
    registry.insert("minecraft:green_banner", GREEN_BANNER.clone());
    registry.insert("minecraft:red_banner", RED_BANNER.clone());
    registry.insert("minecraft:black_banner", BLACK_BANNER.clone());
    registry.insert("minecraft:end_crystal", END_CRYSTAL.clone());
    registry.insert("minecraft:chorus_fruit", CHORUS_FRUIT.clone());
    registry.insert("minecraft:popped_chorus_fruit", POPPED_CHORUS_FRUIT.clone());
    registry.insert("minecraft:beetroot", BEETROOT.clone());
    registry.insert("minecraft:beetroot_seeds", BEETROOT_SEEDS.clone());
    registry.insert("minecraft:beetroot_soup", BEETROOT_SOUP.clone());
    registry.insert("minecraft:dragon_breath", DRAGON_BREATH.clone());
    registry.insert("minecraft:splash_potion", SPLASH_POTION.clone());
    registry.insert("minecraft:spectral_arrow", SPECTRAL_ARROW.clone());
    registry.insert("minecraft:tipped_arrow", TIPPED_ARROW.clone());
    registry.insert("minecraft:lingering_potion", LINGERING_POTION.clone());
    registry.insert("minecraft:shield", SHIELD.clone());
    registry.insert("minecraft:elytra", ELYTRA.clone());
    registry.insert("minecraft:spruce_boat", SPRUCE_BOAT.clone());
    registry.insert("minecraft:birch_boat", BIRCH_BOAT.clone());
    registry.insert("minecraft:jungle_boat", JUNGLE_BOAT.clone());
    registry.insert("minecraft:acacia_boat", ACACIA_BOAT.clone());
    registry.insert("minecraft:dark_oak_boat", DARK_OAK_BOAT.clone());
    registry.insert("minecraft:totem_of_undying", TOTEM_OF_UNDYING.clone());
    registry.insert("minecraft:shulker_shell", SHULKER_SHELL.clone());
    registry.insert("minecraft:iron_nugget", IRON_NUGGET.clone());
    registry.insert("minecraft:knowledge_book", KNOWLEDGE_BOOK.clone());
    registry.insert("minecraft:debug_stick", DEBUG_STICK.clone());
    registry.insert("minecraft:music_disc_13", MUSIC_DISC_13.clone());
    registry.insert("minecraft:music_disc_cat", MUSIC_DISC_CAT.clone());
    registry.insert("minecraft:music_disc_blocks", MUSIC_DISC_BLOCKS.clone());
    registry.insert("minecraft:music_disc_chirp", MUSIC_DISC_CHIRP.clone());
    registry.insert("minecraft:music_disc_far", MUSIC_DISC_FAR.clone());
    registry.insert("minecraft:music_disc_mall", MUSIC_DISC_MALL.clone());
    registry.insert("minecraft:music_disc_mellohi", MUSIC_DISC_MELLOHI.clone());
    registry.insert("minecraft:music_disc_stal", MUSIC_DISC_STAL.clone());
    registry.insert("minecraft:music_disc_strad", MUSIC_DISC_STRAD.clone());
    registry.insert("minecraft:music_disc_ward", MUSIC_DISC_WARD.clone());
    registry.insert("minecraft:music_disc_11", MUSIC_DISC_11.clone());
    registry.insert("minecraft:music_disc_wait", MUSIC_DISC_WAIT.clone());
    registry.insert("minecraft:trident", TRIDENT.clone());
    registry.insert("minecraft:phantom_membrane", PHANTOM_MEMBRANE.clone());
    registry.insert("minecraft:nautilus_shell", NAUTILUS_SHELL.clone());
    registry.insert("minecraft:heart_of_the_sea", HEART_OF_THE_SEA.clone());
    registry.insert("minecraft:crossbow", CROSSBOW.clone());
    registry.insert("minecraft:suspicious_stew", SUSPICIOUS_STEW.clone());
    registry.insert(&blocks::LOOM.registry_name.clone(), LOOM.clone());
    registry.insert(
        "minecraft:flower_banner_pattern",
        FLOWER_BANNER_PATTERN.clone(),
    );
    registry.insert(
        "minecraft:creeper_banner_pattern",
        CREEPER_BANNER_PATTERN.clone(),
    );
    registry.insert(
        "minecraft:skull_banner_pattern",
        SKULL_BANNER_PATTERN.clone(),
    );
    registry.insert(
        "minecraft:mojang_banner_pattern",
        MOJANG_BANNER_PATTERN.clone(),
    );
    registry.insert(
        "minecraft:globe_banner_pattern",
        GLOBE_BANNER_PATTER.clone(),
    );
    registry.insert(&blocks::BARREL.registry_name.clone(), BARREL.clone());
    registry.insert(&blocks::SMOKER.registry_name.clone(), SMOKER.clone());
    registry.insert(
        &blocks::BLAST_FURNACE.registry_name.clone(),
        BLAST_FURNACE.clone(),
    );
    registry.insert(
        &blocks::CARTOGRAPHY_TABLE.registry_name.clone(),
        CARTOGRAPHY_TABLE.clone(),
    );
    registry.insert(
        &blocks::FLETCHING_TABLE.registry_name.clone(),
        FLETCHING_TABLE.clone(),
    );
    registry.insert(
        &blocks::GRINDSTONE.registry_name.clone(),
        GRINDSTONE.clone(),
    );
    registry.insert(&blocks::LECTERN.registry_name.clone(), LECTERN.clone());
    registry.insert(
        &blocks::SMITHING_TABLE.registry_name.clone(),
        SMITHING_TABLE.clone(),
    );
    registry.insert(
        &blocks::STONECUTTER.registry_name.clone(),
        STONECUTTER.clone(),
    );
    registry.insert(&blocks::BELL.registry_name.clone(), BELL.clone());
    registry.insert(&blocks::LANTERN.registry_name.clone(), LANTERN.clone());
    registry.insert("minecraft:sweet_berries", SWEET_BERRIES.clone());
    registry.insert(&blocks::CAMPFIRE.registry_name.clone(), CAMPFIRE.clone());
    registry.insert("minecraft:honeycomb", HONEYCOMB.clone());
    registry.insert(&blocks::BEE_NEST.registry_name.clone(), BEE_NEST.clone());
    registry.insert(&blocks::BEEHIVE.registry_name.clone(), BEEHIVE.clone());
    registry.insert("minecraft:honey_bottle", HONEY_BOTTLE.clone());
    registry.insert(
        &blocks::HONEY_BLOCK.registry_name.clone(),
        HONEY_BLOCK.clone(),
    );
    registry.insert(
        &blocks::HONEYCOMB_BLOCK.registry_name.clone(),
        HONEYCOMB_BLOCK.clone(),
    );
}
