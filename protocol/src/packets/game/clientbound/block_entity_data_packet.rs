use basin2_lib::Nbt;
use crate::network::*;
use crate::packet::*;
use basin2_lib::result::*;
use bytes::BytesMut;

#[derive(PartialEq, Clone, Debug)]
pub struct BlockEntityDataPacket {
    pub pos: BlockPos,
    pub entityType: u8,
    pub tag: Nbt,
}

impl CodablePacket for BlockEntityDataPacket {
    fn encode(self, buf: &mut BytesMut) {
        buf.set_mc_block_pos(self.pos);
        buf.set_mc_u8(self.entityType);
        buf.set_mc_nbt(self.tag);
    }

    fn decode(buf: &mut BytesMut) -> Result<Self>
    where
        Self: Sized,
    {
        let pos = buf.get_mc_block_pos()?;
        let entityType = buf.get_mc_u8()?;
        let tag = buf.get_mc_nbt()?;
        return Ok(BlockEntityDataPacket {
            pos,
            entityType,
            tag,
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::packet::test::*;

    #[test]
    fn test_cycle() -> Result<()> {
        cycle(BlockEntityDataPacket {
            pos: BlockPos {
                x: -100,
                y: 12,
                z: 1024,
            },
            entityType: 12,
            tag: Nbt::make_singleton_compound("test".to_string(), Nbt::Double(12534.0)),
        })
    }
}
