
use super::{ Item, ITEMS, items };
use basin2_lib::Nbt;

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
