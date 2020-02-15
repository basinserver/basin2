use crate::network::*;
use crate::packet::*;
use crate::result::*;
use bytes::BytesMut;

pub struct MoveEntityPosPacket {
    pub entityId: i32,
    pub xa: i16,
    pub ya: i16,
    pub za: i16,
    pub onGround: bool,
}

impl CodablePacket for MoveEntityPosPacket {
    fn encode(self, buf: &mut BytesMut) {
        buf.set_mc_var_int(self.entityId);
        buf.set_mc_i16(self.xa);
        buf.set_mc_i16(self.ya);
        buf.set_mc_i16(self.za);
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
        let onGround = buf.get_mc_bool()?;
        return Ok(MoveEntityPosPacket {
            entityId,
            xa,
            ya,
            za,
            onGround,
        });
    }
}
