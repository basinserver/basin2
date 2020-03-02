use crate::network::*;
use crate::packet::*;
use basin2_lib::result::*;
use bytes::BytesMut;
use basin2_data::{ RecipeSerializer, SimpleCookingSerializer };

struct RecipeSerializerNetworked();

impl RecipeSerializerNetworked {
    fn name(serializer: &RecipeSerializer) -> &'static str {
        use RecipeSerializer::*;

        match serializer {
            CraftingShaped { .. } => "crafting_shaped",
            CraftingShapeless { .. } => "crafting_shapeless",
            CraftingSpecialArmordye => "crafting_special_armordye",
            CraftingSpecialBookcloning => "crafting_special_bookcloning",
            CraftingSpecialMapcloning => "crafting_special_mapcloning",
            CraftingSpecialMapextending => "crafting_special_mapextending",
            CraftingSpecialFireworkRocket => "crafting_special_firework_rocket",
            CraftingSpecialFireworkStar => "crafting_special_firework_star",
            CraftingSpecialFireworkStarFade => "crafting_special_firework_star_fade",
            CraftingSpecialTippedarrow => "crafting_special_tippedarrow",
            CraftingSpecialBannerduplicate => "crafting_special_bannerduplicate",
            CraftingSpecialShielddecoration => "crafting_special_shielddecoration",
            CraftingSpecialShulkerboxcoloring => "crafting_special_shulkerboxcoloring",
            CraftingSpecialSuspiciousstew => "crafting_special_suspiciousstew",
            CraftingSpecialRepairitem => "crafting_special_repairitem",
            Smelting(_) => "smelting",
            Blasting(_) => "blasting",
            Smoking(_) => "smoking",
            CampfireCooking(_) => "campfire_cooking",
            Stonecutting { .. } => "stonecutting",
        }
    }

    fn parse_ingredient(buf: &mut BytesMut) -> Result<Vec<ItemStack>> {
        let count = buf.get_mc_var_int()?;
        let mut items: Vec<ItemStack> = vec![];
        for _ in 0..count {
            items.push(buf.get_mc_item_stack()?);
        }
        Ok(items)
    }

    fn write_ingredient(ingredient: Vec<ItemStack>, buf: &mut BytesMut) {
        buf.set_mc_var_int(ingredient.len() as i32);
        for item in ingredient {
            buf.set_mc_item_stack(item);
        }
    }

    fn parse_shaped(buf: &mut BytesMut) -> Result<RecipeSerializer> {
        use RecipeSerializer::*;

        let width = buf.get_mc_var_int()?;
        let height = buf.get_mc_var_int()?;
        let group = buf.get_mc_string(32767)?;
        let mut recipeItems: Vec<Vec<Vec<ItemStack>>> = vec![];
        for _ in 0..height {
            let mut row: Vec<Vec<ItemStack>> = vec![];
            for _ in 0..width {
                row.push(RecipeSerializerNetworked::parse_ingredient(buf)?);
            }
            recipeItems.push(row);
        }
        let result = buf.get_mc_item_stack()?;
        Ok(CraftingShaped {
            width,
            height,
            group: Some(group),
            recipeItems,
            result,
        })
    }

    fn parse_shapeless(buf: &mut BytesMut) -> Result<RecipeSerializer> {
        use RecipeSerializer::*;

        let group = buf.get_mc_string(32767)?;
        let count = buf.get_mc_var_int()?;
        let mut ingredients: Vec<Vec<ItemStack>> = vec![];
        for _ in 0..count {
            ingredients.push(RecipeSerializerNetworked::parse_ingredient(buf)?);
        }
        let result = buf.get_mc_item_stack()?;
        Ok(CraftingShapeless {
            group: Some(group),
            ingredients,
            result,
        })
    }

    fn parse_cooking(buf: &mut BytesMut) -> Result<SimpleCookingSerializer> {
        let group = buf.get_mc_string(32767)?;
        let ingredient = RecipeSerializerNetworked::parse_ingredient(buf)?;
        let result = buf.get_mc_item_stack()?;
        let experience = buf.get_mc_f32()?;
        let cookingTime = buf.get_mc_var_int()?;
        Ok(SimpleCookingSerializer {
            group: Some(group),
            ingredient,
            result,
            experience,
            cookingTime,
        })
    }

    fn from(value: String, buf: &mut BytesMut) -> Result<RecipeSerializer> {
        use RecipeSerializer::*;

        Ok(match &*value {
            "crafting_shaped" => RecipeSerializerNetworked::parse_shaped(buf)?,
            "crafting_shapeless" => RecipeSerializerNetworked::parse_shapeless(buf)?,
            "crafting_special_armordye" => CraftingSpecialArmordye,
            "crafting_special_bookcloning" => CraftingSpecialBookcloning,
            "crafting_special_mapcloning" => CraftingSpecialMapcloning,
            "crafting_special_mapextending" => CraftingSpecialMapextending,
            "crafting_special_firework_rocket" => CraftingSpecialFireworkRocket,
            "crafting_special_firework_star" => CraftingSpecialFireworkStar,
            "crafting_special_firework_star_fade" => CraftingSpecialFireworkStarFade,
            "crafting_special_tippedarrow" => CraftingSpecialTippedarrow,
            "crafting_special_bannerduplicate" => CraftingSpecialBannerduplicate,
            "crafting_special_shielddecoration" => CraftingSpecialShielddecoration,
            "crafting_special_shulkerboxcoloring" => CraftingSpecialShulkerboxcoloring,
            "crafting_special_suspiciousstew" => CraftingSpecialSuspiciousstew,
            "crafting_special_repairitem" => CraftingSpecialRepairitem,
            "smelting" => Smelting(RecipeSerializerNetworked::parse_cooking(buf)?),
            "blasting" => Blasting(RecipeSerializerNetworked::parse_cooking(buf)?),
            "smoking" => Smoking(RecipeSerializerNetworked::parse_cooking(buf)?),
            "campfire_cooking" => CampfireCooking(RecipeSerializerNetworked::parse_cooking(buf)?),
            "stonecutting" => Stonecutting {
                group: Some(buf.get_mc_string(32767)?),
                ingredient: RecipeSerializerNetworked::parse_ingredient(buf)?,
                result: buf.get_mc_item_stack()?,
            },
            _ => return Err(Box::new(IoError::from(ErrorKind::InvalidData))),
        })
    }
}

#[derive(PartialEq, Clone, Debug)]
pub struct UpdateRecipesPacket {
    pub recipes: Vec<(String, RecipeSerializer)>,
}

impl CodablePacket for UpdateRecipesPacket {
    fn encode(self, buf: &mut BytesMut) {
        use RecipeSerializer::*;

        buf.set_mc_var_int(self.recipes.len() as i32);
        for (id, recipe) in self.recipes {
            buf.set_mc_string(RecipeSerializerNetworked::name(&recipe).to_string());
            buf.set_mc_string(id);
            match recipe {
                CraftingShaped {
                    width,
                    height,
                    group,
                    recipeItems,
                    result,
                } => {
                    buf.set_mc_var_int(width);
                    buf.set_mc_var_int(height);
                    buf.set_mc_string(group.unwrap_or("".to_string()));
                    for ingredient in recipeItems {
                        buf.set_mc_var_int(ingredient.len() as i32);
                        for item in ingredient {
                            RecipeSerializerNetworked::write_ingredient(item, buf);
                        }
                    }
                    buf.set_mc_item_stack(result);
                }
                CraftingShapeless {
                    group,
                    ingredients,
                    result,
                } => {
                    buf.set_mc_string(group.unwrap_or("".to_string()));
                    buf.set_mc_var_int(ingredients.len() as i32);
                    for ingredient in ingredients {
                        RecipeSerializerNetworked::write_ingredient(ingredient, buf);
                    }
                    buf.set_mc_item_stack(result);
                }
                Smelting(cooking)
                | Blasting(cooking)
                | Smoking(cooking)
                | CampfireCooking(cooking) => {
                    buf.set_mc_string(cooking.group.unwrap_or("".to_string()));
                    RecipeSerializerNetworked::write_ingredient(cooking.ingredient, buf);
                    buf.set_mc_item_stack(cooking.result);
                    buf.set_mc_f32(cooking.experience);
                    buf.set_mc_var_int(cooking.cookingTime);
                }
                Stonecutting {
                    group,
                    ingredient,
                    result,
                } => {
                    buf.set_mc_string(group.unwrap_or("".to_string()));
                    RecipeSerializerNetworked::write_ingredient(ingredient, buf);
                    buf.set_mc_item_stack(result);
                }
                _ => (),
            }
        }
    }

    fn decode(buf: &mut BytesMut) -> Result<Self>
    where
        Self: Sized,
    {
        let count = buf.get_mc_var_int()?;
        let mut recipes: Vec<(String, RecipeSerializer)> = vec![];
        for _ in 0..count {
            let serializer = buf.get_mc_string(32767)?;
            let id = buf.get_mc_string(32767)?;
            let serializer = RecipeSerializerNetworked::from(serializer, buf)?;
            recipes.push((id, serializer));
        }
        return Ok(UpdateRecipesPacket { recipes });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::packet::test::*;

    #[test]
    fn test_cycle() -> Result<()> {
        cycle(UpdateRecipesPacket {
            recipes: vec![
                (
                    "shaped recipe".to_string(),
                    RecipeSerializer::CraftingShaped {
                        width: 2,
                        height: 2,
                        group: Some("group".to_string()),
                        recipeItems: vec![
                            vec![ItemStack::empty()],
                            vec![ItemStack::empty()],
                            vec![ItemStack::empty()],
                            vec![ItemStack::empty()],
                        ],
                        result: ItemStack::empty(),
                    },
                ),
                (
                    "shapeless recipe".to_string(),
                    RecipeSerializer::CraftingShapeless {
                        group: Some("group".to_string()),
                        ingredients: vec![
                            vec![ItemStack::empty()],
                            vec![ItemStack::empty()],
                            vec![ItemStack::empty()],
                            vec![ItemStack::empty()],
                        ],
                        result: ItemStack::empty(),
                    },
                ),
                (
                    "simple recipe".to_string(),
                    RecipeSerializer::CraftingSpecialArmordye,
                ),
                (
                    "smelting recipe".to_string(),
                    RecipeSerializer::Smelting(SimpleCookingSerializer {
                        group: Some("group".to_string()),
                        ingredient: vec![ItemStack::empty(), ItemStack::empty()],
                        result: ItemStack::empty(),
                        experience: 120.0,
                        cookingTime: 300,
                    }),
                ),
                (
                    "stonecutting recipe".to_string(),
                    RecipeSerializer::Stonecutting {
                        group: Some("group".to_string()),
                        ingredient: vec![ItemStack::empty(), ItemStack::empty()],
                        result: ItemStack::empty(),
                    },
                ),
            ],
        })
    }
}
