use super::*;


pub const PLAYER_INVENTORY_SIZE: usize = 46;
pub const PLAYER_ENDER_SIZE: usize = 27;

#[derive(Clone)]
pub struct IngamePlayerData {
    pub dimension: DimensionType,
    pub game_type: GameType,
    pub selected_item_slot: i32,
    pub spawn: Option<(i32, i32, i32)>,
    pub spawn_forced: bool,
    pub food_level: i32,
    pub food_exhaustion_level: f32,
    pub food_saturation_level: f32,
    pub food_tick_timer: i32,
    pub xp_level: i32,
    pub xp_total: i32,
    pub xp_seed: i32,
    pub inventory: Vec<ItemStack>,
    pub ender_items: Vec<ItemStack>,
    pub seen_credits: bool,
    pub recipes: Vec<String>,
    pub walk_speed: f32,
    pub fly_speed: f32,
    pub may_fly: bool,
    pub flying: bool,
    pub invulnerable: bool,
    pub may_build: bool,
    pub instabuild: bool,
    pub is_op: bool,
}

impl IngamePlayerData {
    pub fn parse(nbt: &Nbt) -> Result<IngamePlayerData> {
        let dimension = DimensionType::from_i32(nbt.child("Dimension")?.unwrap_i32()?).ok_or(basin_err!("invalid dimension for player"))?;
        let game_type = GameType::from_i32(nbt.child("playerGameType")?.unwrap_i32()?).ok_or(basin_err!("invalid game type for player"))?;
        let spawn = if nbt.child("SpawnY").is_ok() {
            Some((
                nbt.child("SpawnX")?.unwrap_i32()?,
                nbt.child("SpawnY")?.unwrap_i32()?,
                nbt.child("SpawnZ")?.unwrap_i32()?,
            ))
        } else {
            None
        };
        let inventory_items = nbt.child("Inventory")?.unwrap_list()?;
        let mut inventory = vec![ItemStack::empty(); PLAYER_INVENTORY_SIZE];
        for item in inventory_items {
            let slot = item.child("Slot")?.unwrap_i8()?;
            let slot = match slot {
                0..=8 => slot + 36,
                9..=35 => slot,
                100..=103 => (3 - (slot - 100)) + 5,
                -106 => 45,
                _ => return Err(basin_err!("invalid slot value in player inventory: {}", slot)),
            };
            inventory[slot as usize] = ItemStack::try_from(item)?;
        }
        let ender_items_raw = nbt.child("EnderItems")?.unwrap_list()?;
        let mut ender_items = vec![ItemStack::empty(); PLAYER_ENDER_SIZE];
        for item in ender_items_raw {
            let slot = item.child("Slot")?.unwrap_i8()?;
            ender_items[slot as usize] = ItemStack::try_from(item)?;
        }
        let abilities = nbt.child("abilities")?;
        Ok(IngamePlayerData {
            dimension,
            game_type,
            selected_item_slot: nbt.child("SelectedItemSlot")?.unwrap_i32()?,
            spawn,
            spawn_forced: nbt.child("SpawnForced").and_then(|b| b.unwrap_i8()).map(|b| b == 1).unwrap_or(false),
            food_level: nbt.child("foodLevel")?.unwrap_i32()?,
            food_exhaustion_level: nbt.child("foodExhaustionLevel")?.unwrap_f32()?,
            food_saturation_level: nbt.child("foodSaturationLevel")?.unwrap_f32()?,
            food_tick_timer: nbt.child("foodTickTimer")?.unwrap_i32()?,
            xp_level: nbt.child("XpLevel")?.unwrap_i32()?,
            xp_total: nbt.child("XpTotal")?.unwrap_i32()?,
            xp_seed: nbt.child("XpSeed")?.unwrap_i32()?,
            inventory,
            ender_items,
            seen_credits: nbt.child("seenCredits")?.unwrap_i8()? == 1,
            recipes: nbt.child("recipeBook")?.child("recipes")?.unwrap_list()?.iter().map(|item| item.unwrap_str().map(|s| s.to_string()) ).collect::<Result<Vec<String>>>()?,
            walk_speed: abilities.child("walkSpeed")?.unwrap_f32()?,
            fly_speed: abilities.child("flySpeed")?.unwrap_f32()?,
            may_fly: abilities.child("mayfly")?.unwrap_i8()? == 1,
            flying: abilities.child("flying")?.unwrap_i8()? == 1,
            invulnerable: abilities.child("invulnerable")?.unwrap_i8()? == 1,
            may_build: abilities.child("mayBuild")?.unwrap_i8()? == 1,
            instabuild: abilities.child("instabuild")?.unwrap_i8()? == 1,
            is_op: false,
        })
    }
}
