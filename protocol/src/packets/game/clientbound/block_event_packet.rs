
use crate::packet::*;
use crate::network::*;
use bytes::BytesMut;
use uuid::Uuid;
use crate::result::*;

pub struct BlockEventPacket {
    pub pos: BlockPos,
    pub block: undefined,
}

impl CodablePacket for BlockEventPacket {
    fn encode(self, buf: &mut BytesMut) {
        buf.set_mc_block_pos(self.pos);
        buf.set_mc_u8(self.b0);
        buf.set_mc_u8(self.b1);
        // TODO: UNKNOWN: var1.writeVarInt(Registry.BLOCK.getId(this.block));
    }

    fn decode(buf: &mut BytesMut) -> Result<Self> where Self: Sized {
        let pos = buf.get_mc_block_pos()?;
        let b0 = buf.get_mc_u8()?;
        let b1 = buf.get_mc_u8()?;
        // TODO: UNKNOWN: this.block = (Block)Registry.BLOCK.byId(var1.readVarInt());
        return Ok(BlockEventPacket { pos, block });
    }
}
