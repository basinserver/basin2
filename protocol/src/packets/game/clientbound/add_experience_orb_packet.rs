use crate::network::*;
use crate::packet::*;
use crate::result::*;
use bytes::BytesMut;

#[derive(PartialEq, Clone, Debug)]
pub struct AddExperienceOrbPacket {
    pub id: i32,
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub value: i16,
}

impl CodablePacket for AddExperienceOrbPacket {
    fn encode(self, buf: &mut BytesMut) {
        buf.set_mc_var_int(self.id);
        buf.set_mc_f64(self.x);
        buf.set_mc_f64(self.y);
        buf.set_mc_f64(self.z);
        buf.set_mc_i16(self.value);
    }

    fn decode(buf: &mut BytesMut) -> Result<Self>
    where
        Self: Sized,
    {
        let id = buf.get_mc_var_int()?;
        let x = buf.get_mc_f64()?;
        let y = buf.get_mc_f64()?;
        let z = buf.get_mc_f64()?;
        let value = buf.get_mc_i16()?;
        return Ok(AddExperienceOrbPacket { id, x, y, z, value });
    }
}
