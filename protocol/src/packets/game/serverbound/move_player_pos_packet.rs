use crate::network::*;
use crate::packet::*;
use crate::result::*;
use bytes::BytesMut;

pub struct MovePlayerPosPacket {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub onGround: bool,
}

impl CodablePacket for MovePlayerPosPacket {
    fn encode(self, buf: &mut BytesMut) {
        buf.set_mc_f64(self.x);
        buf.set_mc_f64(self.y);
        buf.set_mc_f64(self.z);
        buf.set_mc_bool(self.onGround);
    }

    fn decode(buf: &mut BytesMut) -> Result<Self>
    where
        Self: Sized,
    {
        let x = buf.get_mc_f64()?;
        let y = buf.get_mc_f64()?;
        let z = buf.get_mc_f64()?;
        let onGround = buf.get_mc_bool()?;
        return Ok(MovePlayerPosPacket { x, y, z, onGround });
    }
}
