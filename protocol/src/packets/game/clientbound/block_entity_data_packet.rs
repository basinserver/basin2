use crate::nbt::*;
use crate::network::*;
use crate::packet::*;
use crate::result::*;
use bytes::BytesMut;

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
