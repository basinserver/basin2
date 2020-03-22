use crate::ItemStack;

#[derive(PartialEq, Clone, Debug)]
pub struct SimpleCookingSerializer {
    pub group: Option<String>,
    pub ingredient: Vec<ItemStack>,
    pub result: ItemStack,
    pub experience: f32,
    pub cookingTime: i32,
}

#[derive(PartialEq, Clone, Debug)]
pub enum RecipeSerializer {
    CraftingShaped {
        width: i32,
        height: i32,
        group: Option<String>,
        recipeItems: Vec<Vec<Vec<ItemStack>>>,
        result: ItemStack,
    },
    CraftingShapeless {
        group: Option<String>,
        ingredients: Vec<Vec<ItemStack>>,
        result: ItemStack,
    },
    CraftingSpecialArmordye,
    CraftingSpecialBookcloning,
    CraftingSpecialMapcloning,
    CraftingSpecialMapextending,
    CraftingSpecialFireworkRocket,
    CraftingSpecialFireworkStar,
    CraftingSpecialFireworkStarFade,
    CraftingSpecialTippedarrow,
    CraftingSpecialBannerduplicate,
    CraftingSpecialShielddecoration,
    CraftingSpecialShulkerboxcoloring,
    CraftingSpecialSuspiciousstew,
    CraftingSpecialRepairitem,
    Smelting(SimpleCookingSerializer),
    Blasting(SimpleCookingSerializer),
    Smoking(SimpleCookingSerializer),
    CampfireCooking(SimpleCookingSerializer),
    Stonecutting {
        group: Option<String>,
        ingredient: Vec<ItemStack>,
        result: ItemStack,
    },
}
