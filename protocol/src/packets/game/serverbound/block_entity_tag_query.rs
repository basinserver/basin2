
use crate::packet::*;
use crate::network::*;
use bytes::BytesMut;
use crate::result::*;

pub struct BlockEntityTagQuery {
    pub transactionId: i32,
    pub pos: BlockPos,
}

impl CodablePacket for BlockEntityTagQuery {
    fn encode(self, buf: &mut BytesMut) {
        buf.set_mc_var_int(self.transactionId);
        buf.set_mc_block_pos(self.pos);
    }

    fn decode(buf: &mut BytesMut) -> Result<Self> where Self: Sized {
        let transactionId = buf.get_mc_var_int()?;
        let pos = buf.get_mc_block_pos()?;
        return Ok(BlockEntityTagQuery { transactionId, pos });
    }
}
