
use crate::packet::*;
use crate::network::*;
use bytes::BytesMut;
use uuid::Uuid;
use crate::result::*;

pub struct RecipePacket {
    pub state: RecipePacketState,
    pub recipes: undefined,
    pub toHighlight: undefined,
    pub guiOpen: bool,
    pub filteringCraftable: bool,
    pub furnaceGuiOpen: bool,
    pub furnaceFilteringCraftable: bool,
}

impl CodablePacket for RecipePacket {
    fn encode(self, buf: &mut BytesMut) {
        /* TODO: NOT FOUND */
    }

    fn decode(buf: &mut BytesMut) -> Result<Self> where Self: Sized {
        /* TODO: NOT FOUND */
        return Ok(RecipePacket { state, recipes, toHighlight, guiOpen, filteringCraftable, furnaceGuiOpen, furnaceFilteringCraftable });
    }
}
