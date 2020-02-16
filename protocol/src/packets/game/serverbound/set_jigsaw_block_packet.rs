use crate::network::*;
use crate::packet::*;
use crate::result::*;
use bytes::BytesMut;

#[derive(PartialEq, Clone, Debug)]
pub struct SetJigsawBlockPacket {
    pub pos: BlockPos,
    pub attachmentType: ResourceLocation,
    pub targetPool: ResourceLocation,
    pub finalState: String,
}

impl CodablePacket for SetJigsawBlockPacket {
    fn encode(self, buf: &mut BytesMut) {
        buf.set_mc_block_pos(self.pos);
        buf.set_mc_string(self.attachmentType);
        buf.set_mc_string(self.targetPool);
        buf.set_mc_string(self.finalState);
    }

    fn decode(buf: &mut BytesMut) -> Result<Self>
    where
        Self: Sized,
    {
        let pos = buf.get_mc_block_pos()?;
        let attachmentType = buf.get_mc_string(32767)?;
        let targetPool = buf.get_mc_string(32767)?;
        let finalState = buf.get_mc_string(32767)?;
        return Ok(SetJigsawBlockPacket {
            pos,
            attachmentType,
            targetPool,
            finalState,
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::packet::test::*;

    #[test]
    fn test_cycle() -> Result<()> {
        cycle(SetJigsawBlockPacket {
            pos: BlockPos {
                x: -10,
                y: -20,
                z: -30,
            },
            attachmentType: "unknown".to_string(),
            targetPool: "unknown_pool".to_string(),
            finalState: "final state".to_string(),
        })
    }
}
