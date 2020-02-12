
use crate::packet::*;
use crate::network::*;
use bytes::BytesMut;
use uuid::Uuid;
use crate::result::*;

pub struct PlaceRecipePacket {
    pub containerId: i32,
    pub recipe: ResourceLocation,
    pub shiftDown: bool,
}

impl CodablePacket for PlaceRecipePacket {
    fn encode(self, buf: &mut BytesMut) {
        buf.set_mc_u8(self.containerId);
        buf.set_mc_string(self.recipe);
        buf.set_mc_bool(self.shiftDown);
    }

    fn decode(buf: &mut BytesMut) -> Result<Self> where Self: Sized {
        let containerId = buf.get_mc_u8()?;
        let recipe = buf.get_mc_string()?;
        let shiftDown = buf.get_mc_bool()?;
        return Ok(PlaceRecipePacket { containerId, recipe, shiftDown });
    }
}
