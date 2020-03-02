use std::collections::HashMap;
use basin2_lib::Nbt;
use crate::{ RecipeSerializer, SimpleCookingSerializer, ItemStack };
use serde::{ Serialize, Deserialize };
use serde_json::{ Value, Map };
use std::fs;
use super::*;
use basin2_lib::result::{ Result, * };
use basin2_lib::TryCollect;

#[derive(Serialize, Deserialize)]
pub struct AdvancementDisplayDataIcon {
    item: String,
    nbt: Option<String>,
}

fn true_bool() -> bool {
    true
}

fn false_bool() -> bool {
    false
}

fn empty_vec<T>() -> Vec<T> {
    vec![]
}

fn int_1() -> i32 {
    1
}

fn int_0() -> i32 {
    0
}

#[derive(Serialize, Deserialize)]
pub struct AdvancementDisplayData {
    pub icon: AdvancementDisplayDataIcon,
    pub title: Value, // chat component or string
    pub frame: Option<String>,
    pub background: Option<String>,
    pub description: Value, // chat component or string
    #[serde(default = "true_bool")]
    pub show_toast: bool,
    #[serde(default = "true_bool")]
    pub announce_to_chat: bool,
    #[serde(default = "false_bool")]
    pub hidden: bool,
}

#[derive(Serialize, Deserialize)]
pub struct AdvancementCriteriaData {
    pub trigger: String,
    pub conditions: Option<HashMap<String, Value>>,
}

#[derive(Serialize, Deserialize)]
pub struct AdvancementRewardData {
    #[serde(default = "empty_vec")]
    pub recipes: Vec<String>,
    #[serde(default = "empty_vec")]
    pub loot: Vec<String>,
    pub experience: Option<i32>,
    pub function: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct AdvancementData {
    pub display: Option<AdvancementDisplayData>,
    pub parent: Option<String>,
    pub criteria: HashMap<String, AdvancementCriteriaData>,
    pub requirements: Option<Vec<Vec<String>>>,
    pub rewards: Option<AdvancementRewardData>,
}

type Conditions = Vec<HashMap<String, Value>>;

#[derive(Serialize, Deserialize)]
pub struct LootTableFunction {
    pub function: String,
    #[serde(default = "empty_vec")]
    pub conditions: Conditions,
}

#[derive(Serialize, Deserialize)]
pub struct LootTableEntry {
    #[serde(default = "empty_vec")]
    pub conditions: Conditions,
    #[serde(rename = "type")]
    pub entry_type: String,
    pub name: Option<String>,
    #[serde(default = "empty_vec")]
    pub children: Vec<LootTableEntry>,
    #[serde(default = "false_bool")]
    pub expand: bool,
    #[serde(default = "empty_vec")]
    pub functions: Vec<LootTableFunction>,
    #[serde(default = "int_1")]
    pub weight: i32,
    #[serde(default = "int_0")]
    pub quality: i32,
}

#[derive(Serialize, Deserialize)]
pub struct LootTablePool {
    #[serde(default = "empty_vec")]
    pub conditions: Conditions,
    #[serde(default = "empty_vec")]
    pub functions: Vec<LootTableFunction>,
    pub rolls: Value, // either an int or a {min,max} object
    pub bonus_rolls: Option<Value>, // either an int or a {min,max} object
    #[serde(default = "empty_vec")]
    pub entries: Vec<LootTableEntry>,
}

#[derive(Serialize, Deserialize)]
pub struct LootTableData {
    #[serde(rename = "type")]
    pub data_type: Option<String>,
    #[serde(default = "empty_vec")]
    pub pools: Vec<LootTablePool>,
}

#[derive(Serialize, Deserialize)]
pub struct TagsData {
    #[serde(default = "false_bool")]
    pub replace: bool,
    pub values: Vec<String>,
}

pub struct Data {
    pub advancements: HashMap<String, AdvancementData>,
    pub loot_tables: HashMap<String, LootTableData>,
    pub recipes: HashMap<String, RecipeSerializer>,
    pub structures: HashMap<String, HashMap<String, Nbt>>,
    pub tags_blocks: HashMap<String, TagsData>,
    pub tags_items: HashMap<String, TagsData>,
    pub tags_entity_types: HashMap<String, TagsData>,
    pub tags_fluids: HashMap<String, TagsData>,
}

impl Data {

    fn parse_recipe_shaped(value: Value, tags_items: &mut HashMap<String, TagsData>) -> Result<RecipeSerializer> {
        let value = match value {
            Value::Object(value) => value,
            _ => return Err(basin_err!("not an object")),
        };
        let group = match value.get("group") {
            Some(Value::String(string)) => Some(string.clone()),
            _ => None,
        };
        let result = Data::parse_result(value.get("result"))?;

        let mut rows: Vec<String> = vec![];
        match value.get("pattern") {
            Some(Value::Array(values)) => {
                for value in values {
                    match value {
                        Value::String(string) => {
                            rows.push(string.clone());
                        },
                        _ => return Err(basin_err!("invalid pattern specified: {:?}", value)),
                    }
                }
            },
            _ => return Err(basin_err!("invalid pattern specified: {:?}", value)),
        };
        if rows.len() <= 0 || rows.first().unwrap().len() <= 0 {
            return Err(basin_err!("invalid pattern specified: {:?}", value));
        }
        let keys = match value.get("key") {
            Some(Value::Object(entries)) => {
                entries
            },
            _ => return Err(basin_err!("invalid key specified: {:?}", value)),
        };
        let mut items: Vec<Vec<Vec<ItemStack>>> = vec![];
        for row in rows {
            let mut out: Vec<Vec<ItemStack>> = vec![];
            for key in row.chars() {
                let ingredient = match keys.get(&key.to_string()) {
                    Some(Value::Object(ingredient)) => Data::parse_ingredient(ingredient, tags_items)?,
                    Some(Value::Array(ingredients)) => Data::parse_ingredients(ingredients, tags_items)?,
                    _ => return Err(basin_err!("ingredient is invalid or missing")),
                };
                out.push(ingredient);
            }
            items.push(out);
        }

        return Ok(RecipeSerializer::CraftingShaped {
            width: items.first().unwrap().len() as i32,
            height: items.len() as i32,
            group: group,
            recipeItems: items,
            result,
        })
    }

    fn parse_recipe_shapeless(value: Value, tags_items: &mut HashMap<String, TagsData>) -> Result<RecipeSerializer> {
        let value = match value {
            Value::Object(value) => value,
            _ => return Err(basin_err!("not an object")),
        };
        let group = match value.get("group") {
            Some(Value::String(string)) => Some(string.clone()),
            _ => None,
        };
        let result = Data::parse_result(value.get("result"))?;

        let mut ingredients = vec![];
        match value.get("ingredients") {
            Some(Value::Array(values)) => {
                for value in values {
                    ingredients.push(match value {
                        Value::Object(ingredient) => Data::parse_ingredient(ingredient, tags_items)?,
                        Value::Array(ingredients) => Data::parse_ingredients(ingredients, tags_items)?,
                        _ => return Err(basin_err!("ingredient is invalid or missing")),
                    });
                }
            },
            _ => return Err(basin_err!("invalid pattern specified: {:?}", value)),
        };

        return Ok(RecipeSerializer::CraftingShapeless {
            group: group,
            ingredients,
            result,
        });
    }

    fn parse_ingredient(value: &Map<String, Value>, tags_items: &mut HashMap<String, TagsData>) -> Result<Vec<ItemStack>> {
        match value.get("item") {
            Some(Value::String(id)) => return Ok(vec![ItemStack::from(ItemT::item_not_found(ITEMS.get_str(&*id))?)]),
            None => (),
            _ => return Err(basin_err!("invalid item: {:?}", value)),
        };
        Ok(match value.get("tag") {
            Some(Value::String(tag)) => {
                match tags_items.get(tag) {
                    Some(data) => data.values.iter().map(|name| Ok(ItemStack::from(ItemT::item_not_found(ITEMS.get_str(name))?))).try_collect()?,
                    _ => return Err(basin_err!("invalid tag: {:?}", tag)),
                }
            }
            _ => return Err(basin_err!("invalid item: {:?}", value)),
        })
    }

    fn parse_result(value: Option<&Value>) -> Result<ItemStack> {
        match value {
            Some(Value::String(id)) => {
                Ok(ItemStack::new(ItemT::item_not_found(ITEMS.get_str(&*id))?, 1, None))
            },
            Some(Value::Object(map)) => {
                let item = 
                match map.get("item") {
                    Some(Value::String(id)) => ItemT::item_not_found(ITEMS.get_str(&*id))?,
                    _ => return Err(basin_err!("invalid item: {:?}", map)),
                };
                let count =
                match map.get("count") {
                    Some(Value::Number(count)) => count.as_u64().unwrap_or(1),
                    _ => 1,
                };
                Ok(ItemStack::new(item, count as i32, None))
            },
            _ => return Err(basin_err!("invalid result: {:?}", value)),
        }
    }

    fn parse_ingredients(value: &Vec<Value>, tags_items: &mut HashMap<String, TagsData>) -> Result<Vec<ItemStack>> {
        let mut out = vec![];
        for ingredient in value {
            let ingredient = match ingredient {
                Value::Object(ingredient) => Data::parse_ingredient(ingredient, tags_items)?,
                _ => return Err(basin_err!("not an object")),
            };
            out.extend(ingredient);
        }
        Ok(out)
    }

    fn single_ingredient_base(value: &Map<String, Value>, tags_items: &mut HashMap<String, TagsData>) -> Result<(Option<String>, Vec<ItemStack>, ItemStack)> {
        let group = match value.get("group") {
            Some(Value::String(string)) => Some(string.clone()),
            _ => None,
        };
        let ingredient = match value.get("ingredient") {
            Some(Value::Object(ingredient)) => Data::parse_ingredient(ingredient, tags_items)?,
            Some(Value::Array(ingredients)) => Data::parse_ingredients(ingredients, tags_items)?,
            _ => return Err(basin_err!("ingredient is invalid or missing")),
        };
        let result = Data::parse_result(value.get("result"))?;

        return Ok((group, ingredient, result));
    }

    fn parse_recipe_cooking(value: Value, tags_items: &mut HashMap<String, TagsData>) -> Result<SimpleCookingSerializer> {
        let value = match value {
            Value::Object(value) => value,
            _ => return Err(basin_err!("not an object")),
        };
        let (group, ingredient, result) = Data::single_ingredient_base(&value, tags_items)?;
        let experience = match value.get("experience") {
            Some(Value::Number(num)) => num.as_f64().unwrap_or(0.0) as f32,
            _ => return Err( basin_err!("experience is invalid or missing")),
        };
        let cookingTime = match value.get("cookingtime") {
            Some(Value::Number(num)) => num.as_i64().unwrap_or(100) as i32,
            _ => 100,
        };

        Ok(SimpleCookingSerializer {
            group,
            ingredient,
            result,
            experience,
            cookingTime,
        })
    }

    fn parse_recipe_stonecutting(value: Value, tags_items: &mut HashMap<String, TagsData>) -> Result<RecipeSerializer> {
        let value = match value {
            Value::Object(value) => value,
            _ => return Err(basin_err!("not an object")),
        };
        let (group, ingredient, result) = Data::single_ingredient_base(&value, tags_items)?;

        Ok(RecipeSerializer::Stonecutting {
            group,
            ingredient,
            result,
        })
    }

    fn parse_recipe(value: Value, tags_items: &mut HashMap<String, TagsData>) -> Result<RecipeSerializer> {
        use RecipeSerializer::*;
        let recipe_type = match &value {
            Value::Object(map) => {
                match map.get("type") {
                    Some(Value::String(string)) => Some(string),
                    _ => None,
                }
            },
            _ => None,
        };
        if recipe_type.is_none() {
            return Err(basin_err!("no recipe type in recipe"));
        }
        let recipe_type = recipe_type.unwrap();

        Ok(match &**recipe_type {
            "minecraft:crafting_shaped" => Data::parse_recipe_shaped(value, tags_items)?,
            "minecraft:crafting_shapeless" => Data::parse_recipe_shapeless(value, tags_items)?,
            "minecraft:crafting_special_armordye" => CraftingSpecialArmordye,
            "minecraft:crafting_special_bookcloning" => CraftingSpecialBookcloning,
            "minecraft:crafting_special_mapcloning" => CraftingSpecialMapcloning,
            "minecraft:crafting_special_mapextending" => CraftingSpecialMapextending,
            "minecraft:crafting_special_firework_rocket" => CraftingSpecialFireworkRocket,
            "minecraft:crafting_special_firework_star" => CraftingSpecialFireworkStar,
            "minecraft:crafting_special_firework_star_fade" => CraftingSpecialFireworkStarFade,
            "minecraft:crafting_special_tippedarrow" => CraftingSpecialTippedarrow,
            "minecraft:crafting_special_bannerduplicate" => CraftingSpecialBannerduplicate,
            "minecraft:crafting_special_shielddecoration" => CraftingSpecialShielddecoration,
            "minecraft:crafting_special_shulkerboxcoloring" => CraftingSpecialShulkerboxcoloring,
            "minecraft:crafting_special_suspiciousstew" => CraftingSpecialSuspiciousstew,
            "minecraft:crafting_special_repairitem" => CraftingSpecialRepairitem,
            "minecraft:smelting" => Smelting(Data::parse_recipe_cooking(value, tags_items)?),
            "minecraft:blasting" => Blasting(Data::parse_recipe_cooking(value, tags_items)?),
            "minecraft:smoking" => Smoking(Data::parse_recipe_cooking(value, tags_items)?),
            "minecraft:campfire_cooking" => CampfireCooking(Data::parse_recipe_cooking(value, tags_items)?),
            "minecraft:stonecutting" => Data::parse_recipe_stonecutting(value, tags_items)?,
            _ => return Err(basin_err!("invalid recipe_type: {}", &**recipe_type)),
        })
    }

    fn load_tags(namespace: &str, sub: &str, tags: &mut HashMap<String, TagsData>) -> Result<()> {
        for item in fs::read_dir(format!("./data/{}/tags/{}/", namespace, sub))? {
            let item = item?;
            let filename = item.file_name();
            let filename = filename.to_str().unwrap();
            if !filename.ends_with(".json") {
                continue;
            }
            let filename = &filename[0..filename.len() - 5];
            let raw = fs::read_to_string(item.path())?;
            let data: TagsData = serde_json::from_str(&*raw)?;
            tags.insert(format!("{}:{}", namespace, filename), data);
        }
        Ok(())
    }

    fn load_namespace(
        namespace: &str,
        advancements: &mut HashMap<String, AdvancementData>,
        loot_tables: &mut HashMap<String, LootTableData>,
        recipes: &mut HashMap<String, RecipeSerializer>,
        structures: &mut HashMap<String, HashMap<String, Nbt>>,
        tags_blocks: &mut HashMap<String, TagsData>,
        tags_items: &mut HashMap<String, TagsData>,
        tags_entity_types: &mut HashMap<String, TagsData>,
        tags_fluids: &mut HashMap<String, TagsData>,
    ) -> Result<()> {

        Data::load_tags(namespace, "blocks", tags_blocks)?;
        Data::load_tags(namespace, "items", tags_items)?;
        Data::load_tags(namespace, "entity_types", tags_entity_types)?;
        Data::load_tags(namespace, "fluids", tags_fluids)?;

        for root in fs::read_dir(format!("./data/{}/advancements/", namespace))? {
            let root = root?;
            let path = root.path();
            if !path.is_dir() {
                continue;
            }
            let root_name = root.file_name();
            let root_name = root_name.to_str().unwrap();
            for item in fs::read_dir(format!("./data/{}/advancements/{}", namespace, root_name))? {
                let item = item?;
                let filename = item.file_name();
                let filename = filename.to_str().unwrap();
                if !filename.ends_with(".json") {
                    continue;
                }
                let filename = &filename[0..filename.len() - 5];
                let raw = fs::read_to_string(item.path())?;
                let advancement: AdvancementData = serde_json::from_str(&*raw)?;
                advancements.insert(format!("{}:{}/{}", namespace, root_name, filename), advancement);
            }
        }

        for root in fs::read_dir(format!("./data/{}/loot_tables/", namespace))? {
            let root = root?;
            let path = root.path();
            if !path.is_dir() {
                continue;
            }
            let root_name = root.file_name();
            let root_name = root_name.to_str().unwrap();
            for item in fs::read_dir(format!("./data/{}/loot_tables/{}", namespace, root_name))? {
                let item = item?;
                let filename = item.file_name();
                let filename = filename.to_str().unwrap();
                if !filename.ends_with(".json") {
                    continue;
                }
                let filename = &filename[0..filename.len() - 5];
                let raw = fs::read_to_string(item.path())?;
                let loot_table: LootTableData = serde_json::from_str(&*raw)?;
                loot_tables.insert(format!("{}:{}/{}", namespace, root_name, filename), loot_table);
            }
        }

        for item in fs::read_dir(format!("./data/{}/recipes/", namespace))? {
            let item = item?;
            let filename = item.file_name();
            let filename = filename.to_str().unwrap();
            if !filename.ends_with(".json") {
                continue;
            }
            let filename = &filename[0..filename.len() - 5];
            let raw = fs::read_to_string(item.path())?;
            let raw_recipe: Value = serde_json::from_str(&*raw)?;
            let recipe = Data::parse_recipe(raw_recipe, tags_items)?;
            recipes.insert(format!("{}:{}", namespace, filename), recipe);
        }

        // TODO: structures

        Ok(())
    }

    pub fn load() -> Result<Data> {
        let mut advancements: HashMap<String, AdvancementData> = HashMap::new();
        let mut loot_tables: HashMap<String, LootTableData> = HashMap::new();
        let mut recipes: HashMap<String, RecipeSerializer> = HashMap::new();
        let mut structures: HashMap<String, HashMap<String, Nbt>> = HashMap::new();
        let mut tags_blocks: HashMap<String, TagsData> = HashMap::new();    
        let mut tags_items: HashMap<String, TagsData> = HashMap::new();    
        let mut tags_entity_types: HashMap<String, TagsData> = HashMap::new();    
        let mut tags_fluids: HashMap<String, TagsData> = HashMap::new();    
        for namespace in fs::read_dir("./data/")? {
            let namespace = namespace?;
            let path = namespace.path();
            if path.is_dir() {
                let filename = path.file_name().unwrap();
                Data::load_namespace(
                    filename.to_str().unwrap(),
                    &mut advancements,
                    &mut loot_tables,
                    &mut recipes,
                    &mut structures,
                    &mut tags_blocks,
                    &mut tags_items,
                    &mut tags_entity_types,
                    &mut tags_fluids,
                )?;
            }
        }
        Ok(Data {
            advancements,
            loot_tables,
            recipes,
            structures,
            tags_blocks,
            tags_items,
            tags_entity_types,
            tags_fluids,
        })
    }
}
