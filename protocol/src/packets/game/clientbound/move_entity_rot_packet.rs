use crate::network::*;
use crate::packet::*;
use crate::result::*;
use bytes::BytesMut;

pub struct MoveEntityRotPacket {
    pub entityId: i32,
    pub yRot: i8,
    pub xRot: i8,
    pub onGround: bool,
}

impl CodablePacket for MoveEntityRotPacket {
    fn encode(self, buf: &mut BytesMut) {
        buf.set_mc_var_int(self.entityId);
        buf.set_mc_i8(self.yRot);
        buf.set_mc_i8(self.xRot);
        buf.set_mc_bool(self.onGround);
    }

    fn decode(buf: &mut BytesMut) -> Result<Self>
    where
        Self: Sized,
    {
        let entityId = buf.get_mc_var_int()?;
        let yRot = buf.get_mc_i8()?;
        let xRot = buf.get_mc_i8()?;
        let onGround = buf.get_mc_bool()?;
        return Ok(MoveEntityRotPacket {
            entityId,
            yRot,
            xRot,
            onGround,
        });
    }
}
