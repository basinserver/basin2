
use crate::packet::*;
use crate::network::*;
use bytes::BytesMut;
use crate::result::*;

pub struct TeleportEntityPacket {
    pub id: i32,
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub yRot: u8,
    pub xRot: u8,
    pub onGround: bool,
}

impl CodablePacket for TeleportEntityPacket {
    fn encode(self, buf: &mut BytesMut) {
        buf.set_mc_var_int(self.id);
        buf.set_mc_f64(self.x);
        buf.set_mc_f64(self.y);
        buf.set_mc_f64(self.z);
        buf.set_mc_u8(self.yRot);
        buf.set_mc_u8(self.xRot);
        buf.set_mc_bool(self.onGround);
    }

    fn decode(buf: &mut BytesMut) -> Result<Self> where Self: Sized {
        let id = buf.get_mc_var_int()?;
        let x = buf.get_mc_f64()?;
        let y = buf.get_mc_f64()?;
        let z = buf.get_mc_f64()?;
        let yRot = buf.get_mc_u8()?;
        let xRot = buf.get_mc_u8()?;
        let onGround = buf.get_mc_bool()?;
        return Ok(TeleportEntityPacket { id, x, y, z, yRot, xRot, onGround });
    }
}
