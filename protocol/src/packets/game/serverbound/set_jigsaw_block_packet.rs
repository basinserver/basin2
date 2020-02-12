
use crate::packet::*;
use crate::network::*;
use bytes::BytesMut;
use uuid::Uuid;
use crate::result::*;

pub struct SetJigsawBlockPacket {
    pub pos: BlockPos,
    pub attachementType: ResourceLocation,
    pub targetPool: ResourceLocation,
    pub finalState: String,
}

impl CodablePacket for SetJigsawBlockPacket {
    fn encode(self, buf: &mut BytesMut) {
        buf.set_mc_block_pos(self.pos);
        buf.set_mc_string(self.attachementType);
        buf.set_mc_string(self.targetPool);
        buf.set_mc_string(self.finalState);
    }

    fn decode(buf: &mut BytesMut) -> Result<Self> where Self: Sized {
        let pos = buf.get_mc_block_pos()?;
        let attachementType = buf.get_mc_string()?;
        let targetPool = buf.get_mc_string()?;
        let finalState = buf.get_mc_string_bounded(32767)?;
        return Ok(SetJigsawBlockPacket { pos, attachementType, targetPool, finalState });
    }
}
