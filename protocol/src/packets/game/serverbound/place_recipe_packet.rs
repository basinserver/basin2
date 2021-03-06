use crate::network::*;
use crate::packet::*;
use basin2_lib::result::*;
use bytes::BytesMut;

#[derive(PartialEq, Clone, Debug)]
pub struct PlaceRecipePacket {
    pub containerId: u8,
    pub recipe: ResourceLocation,
    pub shiftDown: bool,
}

impl CodablePacket for PlaceRecipePacket {
    fn encode(self, buf: &mut BytesMut) {
        buf.set_mc_u8(self.containerId);
        buf.set_mc_string(self.recipe);
        buf.set_mc_bool(self.shiftDown);
    }

    fn decode(buf: &mut BytesMut) -> Result<Self>
    where
        Self: Sized,
    {
        let containerId = buf.get_mc_u8()?;
        let recipe = buf.get_mc_string(32767)?;
        let shiftDown = buf.get_mc_bool()?;
        return Ok(PlaceRecipePacket {
            containerId,
            recipe,
            shiftDown,
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::packet::test::*;

    #[test]
    fn test_cycle() -> Result<()> {
        cycle(PlaceRecipePacket {
            containerId: 34,
            recipe: "testRecipe".to_string(),
            shiftDown: true,
        })
    }
}
