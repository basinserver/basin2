
use super::{ Item, ITEMS, items };
use basin2_lib::Nbt;
use basin2_lib::result::*;
use std::convert::TryFrom;

#[derive(PartialEq, Clone, Debug)]
pub struct ItemStack {
    pub count: i32,
    pub item: Item,
    pub nbt: Option<Nbt>,
}

impl From<Item> for ItemStack {
    fn from(item: Item) -> ItemStack {
        ItemStack {
            count: 1,
            item,
            nbt: None,
        }
    }
}

impl From<&str> for ItemStack {
    fn from(item: &str) -> ItemStack {
        ItemStack {
            count: 1,
            item: ITEMS.get_str(item).unwrap_or(items::AIR.clone()),
            nbt: None,
        }
    }
}


impl TryFrom<&Nbt> for ItemStack {
    type Error = Error;

    fn try_from(item: &Nbt) -> Result<ItemStack> {
        Ok(ItemStack {
            count: item.child("Count").and_then(|count| count.unwrap_i8() ).unwrap_or(1) as i32,
            item: ITEMS.get_str(item.child("id").and_then(|id| id.unwrap_str() ).unwrap_or("minecraft:stone")).unwrap_or(items::STONE.clone()),
            nbt: item.child("tag").ok().cloned(),
        })
    }
}

impl ItemStack {
    pub fn new(item: Item, count: i32, nbt: Option<Nbt>) -> ItemStack {
        ItemStack {
            item,
            count,
            nbt,
        }
    }

    pub fn is_empty(&self) -> bool {
        return self.count <= 0 || *self.item.registry_name == "minecraft:air";
    }

    pub fn empty() -> ItemStack {
        ItemStack {
            item: ITEMS.get_str("minecraft:air").unwrap(),
            count: 0,
            nbt: None,
        }
    }
}
