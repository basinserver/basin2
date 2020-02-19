use std::collections::HashMap;
use basin2_protocol::{Nbt, network::ChatComponent};
use serde::{ Serialize, Deserialize };
use serde_json::Value;

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
    pub conditions: HashMap<String, Value>,
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
pub struct LootTablePool {
    #[serde(default = "empty_vec")]
    pub conditions: Conditions,
    #[serde(default = "empty_vec")]
    pub functions: Vec<LootTableFunction>,
    pub rolls: Value, // either an int or a {min,max} object
    pub bonus_rolls: Value, // either an int or a {min,max} object
    #[serde(default = "empty_vec")]
    pub entries: Vec<LootTableEntry>,
}

#[derive(Serialize, Deserialize)]
pub struct LootTableData {
    #[serde(rename = "type")]
    pub data_type: Option<String>,
    pub pools: Vec<LootTablePool>,
}

#[derive(Serialize, Deserialize)]
pub struct RecipeData {
    
}

#[derive(Serialize, Deserialize)]
pub struct TagsData {

}

pub struct Data {
    advancements: HashMap<String, AdvancementData>,
    loot_tables: HashMap<String, LootTableData>,
    recipes: HashMap<String, RecipeData>,
    structures: HashMap<String, HashMap<String, Nbt>>,
    tags: HashMap<String, TagsData>,
}

fn load_data() {

}