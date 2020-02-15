use crate::network::*;
use crate::packet::*;
use crate::result::*;
use bytes::BytesMut;

pub struct SignUpdatePacket {
    pub pos: BlockPos,
    pub lines: (String, String, String, String),
}

impl CodablePacket for SignUpdatePacket {
    fn encode(self, buf: &mut BytesMut) {
        buf.set_mc_block_pos(self.pos);
        buf.set_mc_string(self.lines.0);
        buf.set_mc_string(self.lines.1);
        buf.set_mc_string(self.lines.2);
        buf.set_mc_string(self.lines.3);
    }

    fn decode(buf: &mut BytesMut) -> Result<Self>
    where
        Self: Sized,
    {
        let pos = buf.get_mc_block_pos()?;
        let lines = (
            buf.get_mc_string(384)?,
            buf.get_mc_string(384)?,
            buf.get_mc_string(384)?,
            buf.get_mc_string(384)?,
        );
        return Ok(SignUpdatePacket { pos, lines });
    }
}
