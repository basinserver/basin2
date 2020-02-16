use crate::network::*;
use crate::packet::*;
use crate::result::*;
use bytes::BytesMut;

#[derive(PartialEq, Clone, Debug)]
pub struct MoveEntityPosRotPacket {
    pub entityId: i32,
    pub xa: i16,
    pub ya: i16,
    pub za: i16,
    pub yRot: i8,
    pub xRot: i8,
    pub onGround: bool,
}

impl CodablePacket for MoveEntityPosRotPacket {
    fn encode(self, buf: &mut BytesMut) {
        buf.set_mc_var_int(self.entityId);
        buf.set_mc_i16(self.xa);
        buf.set_mc_i16(self.ya);
        buf.set_mc_i16(self.za);
        buf.set_mc_i8(self.yRot);
        buf.set_mc_i8(self.xRot);
        buf.set_mc_bool(self.onGround);
    }

    fn decode(buf: &mut BytesMut) -> Result<Self>
    where
        Self: Sized,
    {
        let entityId = buf.get_mc_var_int()?;
        let xa = buf.get_mc_i16()?;
        let ya = buf.get_mc_i16()?;
        let za = buf.get_mc_i16()?;
        let yRot = buf.get_mc_i8()?;
        let xRot = buf.get_mc_i8()?;
        let onGround = buf.get_mc_bool()?;
        return Ok(MoveEntityPosRotPacket {
            entityId,
            xa,
            ya,
            za,
            yRot,
            xRot,
            onGround,
        });
    }
}
