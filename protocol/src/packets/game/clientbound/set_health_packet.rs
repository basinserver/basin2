
use crate::packet::*;
use crate::network::*;
use bytes::BytesMut;
use uuid::Uuid;
use crate::result::*;

pub struct SetHealthPacket {
    pub health: f32,
    pub food: i32,
    pub saturation: f32,
}

impl CodablePacket for SetHealthPacket {
    fn encode(self, buf: &mut BytesMut) {
        buf.set_mc_f32(self.health);
        buf.set_mc_var_int(self.food);
        buf.set_mc_f32(self.saturation);
    }

    fn decode(buf: &mut BytesMut) -> Result<Self> where Self: Sized {
        let health = buf.get_mc_f32()?;
        let food = buf.get_mc_var_int()?;
        let saturation = buf.get_mc_f32()?;
        return Ok(SetHealthPacket { health, food, saturation });
    }
}
