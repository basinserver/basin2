
use crate::packet::*;
use crate::network::*;
use bytes::BytesMut;
use uuid::Uuid;
use crate::result::*;

pub struct GameEventPacket {
    pub event: i32,
    pub param: f32,
}

impl CodablePacket for GameEventPacket {
    fn encode(self, buf: &mut BytesMut) {
        buf.set_mc_u8(self.event);
        buf.set_mc_f32(self.param);
    }

    fn decode(buf: &mut BytesMut) -> Result<Self> where Self: Sized {
        let event = buf.get_mc_u8()?;
        let param = buf.get_mc_f32()?;
        return Ok(GameEventPacket { event, param });
    }
}
