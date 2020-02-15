
use crate::packet::*;
use crate::network::*;
use bytes::BytesMut;
use uuid::Uuid;
use crate::result::*;

pub struct MovePlayerPacket {
    pub onGround: bool,
}

impl CodablePacket for MovePlayerPacket {
    fn encode(self, buf: &mut BytesMut) {
        buf.set_mc_bool(self.onGround);
    }

    fn decode(buf: &mut BytesMut) -> Result<Self> where Self: Sized {
        let onGround = buf.get_mc_bool()?;
        return Ok(MovePlayerPacket { onGround });
    }
}
