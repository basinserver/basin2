
use crate::packet::*;
use crate::network::*;
use bytes::BytesMut;
use uuid::Uuid;
use crate::result::*;

pub struct SignUpdatePacket {
    pub pos: BlockPos,
    pub lines: Vec<String>,
}

impl CodablePacket for SignUpdatePacket {
    fn encode(self, buf: &mut BytesMut) {
        /* TODO: NOT FOUND */
    }

    fn decode(buf: &mut BytesMut) -> Result<Self> where Self: Sized {
        /* TODO: NOT FOUND */
        return Ok(SignUpdatePacket { pos, lines });
    }
}
