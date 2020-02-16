use crate::network::*;
use crate::packet::*;
use crate::result::*;
use bytes::BytesMut;

#[derive(PartialEq, Clone, Debug)]
pub struct BlockDestructionPacket {
    pub id: i32,
    pub pos: BlockPos,
    pub progress: u8,
}

impl CodablePacket for BlockDestructionPacket {
    fn encode(self, buf: &mut BytesMut) {
        buf.set_mc_var_int(self.id);
        buf.set_mc_block_pos(self.pos);
        buf.set_mc_u8(self.progress);
    }

    fn decode(buf: &mut BytesMut) -> Result<Self>
    where
        Self: Sized,
    {
        let id = buf.get_mc_var_int()?;
        let pos = buf.get_mc_block_pos()?;
        let progress = buf.get_mc_u8()?;
        return Ok(BlockDestructionPacket { id, pos, progress });
    }
}
