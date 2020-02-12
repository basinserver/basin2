
use crate::packet::*;
use crate::network::*;
use bytes::BytesMut;
use uuid::Uuid;
use crate::result::*;

pub struct RotateHeadPacket {
    pub entityId: i32,
    pub yHeadRot: u8,
}

impl CodablePacket for RotateHeadPacket {
    fn encode(self, buf: &mut BytesMut) {
        buf.set_mc_var_int(self.entityId);
        buf.set_mc_u8(self.yHeadRot);
    }

    fn decode(buf: &mut BytesMut) -> Result<Self> where Self: Sized {
        let entityId = buf.get_mc_var_int()?;
        let yHeadRot = buf.get_mc_u8()?;
        return Ok(RotateHeadPacket { entityId, yHeadRot });
    }
}
