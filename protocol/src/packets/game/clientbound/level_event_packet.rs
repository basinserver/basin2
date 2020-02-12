
use crate::packet::*;
use crate::network::*;
use bytes::BytesMut;
use uuid::Uuid;
use crate::result::*;

pub struct LevelEventPacket {
    pub type: i32,
    pub pos: BlockPos,
    pub data: i32,
    pub globalEvent: bool,
}

impl CodablePacket for LevelEventPacket {
    fn encode(self, buf: &mut BytesMut) {
        buf.set_mc_i32(self.type);
        buf.set_mc_block_pos(self.pos);
        buf.set_mc_i32(self.data);
        buf.set_mc_bool(self.globalEvent);
    }

    fn decode(buf: &mut BytesMut) -> Result<Self> where Self: Sized {
        let type = buf.get_mc_i32()?;
        let pos = buf.get_mc_block_pos()?;
        let data = buf.get_mc_i32()?;
        let globalEvent = buf.get_mc_bool()?;
        return Ok(LevelEventPacket { type, pos, data, globalEvent });
    }
}
