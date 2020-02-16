use crate::network::*;
use crate::packet::*;
use crate::result::*;
use bytes::BytesMut;

#[derive(PartialEq, Clone, Debug)]
pub struct PlaceGhostRecipePacket {
    pub containerId: u8,
    pub recipe: ResourceLocation,
}

impl CodablePacket for PlaceGhostRecipePacket {
    fn encode(self, buf: &mut BytesMut) {
        buf.set_mc_u8(self.containerId);
        buf.set_mc_string(self.recipe);
    }

    fn decode(buf: &mut BytesMut) -> Result<Self>
    where
        Self: Sized,
    {
        let containerId = buf.get_mc_u8()?;
        let recipe = buf.get_mc_string(32767)?;
        return Ok(PlaceGhostRecipePacket {
            containerId,
            recipe,
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::packet::test::*;

    #[test]
    fn test_cycle() -> Result<()> {
        cycle(PlaceGhostRecipePacket {
            containerId: 45,
            recipe: "some recipe".to_string(),
        })
    }
}