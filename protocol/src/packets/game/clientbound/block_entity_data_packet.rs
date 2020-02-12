
use crate::packet::*;
use crate::network::*;
use bytes::BytesMut;
use uuid::Uuid;
use crate::result::*;

pub struct BlockEntityDataPacket {
    pub pos: BlockPos,
    pub type: i32,
    pub tag: Nbt,
}

impl CodablePacket for BlockEntityDataPacket {
    fn encode(self, buf: &mut BytesMut) {
        buf.set_mc_block_pos(self.pos);
        // TODO: UNKNOWN: var1.writeByte((byte)this.type);
        buf.set_mc_nbt(self.tag);
    }

    fn decode(buf: &mut BytesMut) -> Result<Self> where Self: Sized {
        let pos = buf.get_mc_block_pos()?;
        let type = buf.get_mc_u8()?;
        let tag = buf.get_mc_nbt()?;
        return Ok(BlockEntityDataPacket { pos, type, tag });
    }
}
