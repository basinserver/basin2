
use crate::packet::*;
use crate::network::*;
use bytes::BytesMut;
use uuid::Uuid;
use crate::result::*;

pub struct AddGlobalEntityPacket {
    pub id: i32,
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub type: i32,
}

impl CodablePacket for AddGlobalEntityPacket {
    fn encode(self, buf: &mut BytesMut) {
        buf.set_mc_var_int(self.id);
        buf.set_mc_u8(self.type);
        buf.set_mc_f64(self.x);
        buf.set_mc_f64(self.y);
        buf.set_mc_f64(self.z);
    }

    fn decode(buf: &mut BytesMut) -> Result<Self> where Self: Sized {
        let id = buf.get_mc_var_int()?;
        let type = buf.get_mc_u8()?;
        let x = buf.get_mc_f64()?;
        let y = buf.get_mc_f64()?;
        let z = buf.get_mc_f64()?;
        return Ok(AddGlobalEntityPacket { id, x, y, z, type });
    }
}
