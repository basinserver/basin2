use crate::loader::LootTableData;

#[derive(Debug, Clone)]
pub struct LootTable {

}

impl From<LootTableData> for LootTable {
    fn from(data: LootTableData) -> LootTable {
        //TODO: nyi
        LootTable {}
    }
}