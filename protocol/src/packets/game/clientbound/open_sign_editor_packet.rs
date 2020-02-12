
use crate::packet::*;
use crate::network::*;
use bytes::BytesMut;
use uuid::Uuid;
use crate::result::*;

pub struct OpenSignEditorPacket {
    pub pos: BlockPos,
}

impl CodablePacket for OpenSignEditorPacket {
    fn encode(self, buf: &mut BytesMut) {
        buf.set_mc_block_pos(self.pos);
    }

    fn decode(buf: &mut BytesMut) -> Result<Self> where Self: Sized {
        let pos = buf.get_mc_block_pos()?;
        return Ok(OpenSignEditorPacket { pos });
    }
}
