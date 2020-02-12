
use crate::packet::*;
use crate::network::*;
use bytes::BytesMut;
use uuid::Uuid;
use crate::result::*;

pub struct MoveEntityPacket {
    pub entityId: i32,
    pub xa: i16,
    pub ya: i16,
    pub za: i16,
    pub yRot: u8,
    pub xRot: u8,
    pub onGround: bool,
    pub hasRot: bool,
    pub hasPos: bool,
}

impl CodablePacket for MoveEntityPacket {
    fn encode(self, buf: &mut BytesMut) {
        buf.set_mc_var_int(self.entityId);
    }

    fn decode(buf: &mut BytesMut) -> Result<Self> where Self: Sized {
        let entityId = buf.get_mc_var_int()?;
        return Ok(MoveEntityPacket { entityId, xa, ya, za, yRot, xRot, onGround, hasRot, hasPos });
    }
}
