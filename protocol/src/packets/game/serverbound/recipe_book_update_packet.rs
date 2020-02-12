
use crate::packet::*;
use crate::network::*;
use bytes::BytesMut;
use uuid::Uuid;
use crate::result::*;

pub struct RecipeBookUpdatePacket {
    pub purpose: RecipeBookUpdatePacketPurpose,
    pub recipe: ResourceLocation,
    pub guiOpen: bool,
    pub filteringCraftable: bool,
    pub furnaceGuiOpen: bool,
    pub furnaceFilteringCraftable: bool,
    pub blastFurnaceGuiOpen: bool,
    pub blastFurnaceFilteringCraftable: bool,
    pub smokerGuiOpen: bool,
    pub smokerFilteringCraftable: bool,
}

impl CodablePacket for RecipeBookUpdatePacket {
    fn encode(self, buf: &mut BytesMut) {
        /* TODO: NOT FOUND */
    }

    fn decode(buf: &mut BytesMut) -> Result<Self> where Self: Sized {
        /* TODO: NOT FOUND */
        return Ok(RecipeBookUpdatePacket { purpose, recipe, guiOpen, filteringCraftable, furnaceGuiOpen, furnaceFilteringCraftable, blastFurnaceGuiOpen, blastFurnaceFilteringCraftable, smokerGuiOpen, smokerFilteringCraftable });
    }
}
