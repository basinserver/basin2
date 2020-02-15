use crate::network::*;
use crate::packet::*;
use crate::result::*;
use bytes::BytesMut;

pub struct BlockEventPacket {
    pub pos: BlockPos,
    pub b0: u8,
    pub b1: u8,
    pub block: Block,
}

impl CodablePacket for BlockEventPacket {
    fn encode(self, buf: &mut BytesMut) {
        buf.set_mc_block_pos(self.pos);
        buf.set_mc_u8(self.b0);
        buf.set_mc_u8(self.b1);
        buf.set_mc_var_int(self.block);
    }

    fn decode(buf: &mut BytesMut) -> Result<Self>
    where
        Self: Sized,
    {
        let pos = buf.get_mc_block_pos()?;
        let b0 = buf.get_mc_u8()?;
        let b1 = buf.get_mc_u8()?;
        let block = buf.get_mc_var_int()?;
        return Ok(BlockEventPacket { pos, b0, b1, block });
    }
}
