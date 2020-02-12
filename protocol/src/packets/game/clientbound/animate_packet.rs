
use crate::packet::*;
use crate::network::*;
use bytes::BytesMut;
use uuid::Uuid;
use crate::result::*;

pub struct AnimatePacket {
    pub id: i32,
    pub action: i32,
}

impl CodablePacket for AnimatePacket {
    fn encode(self, buf: &mut BytesMut) {
        buf.set_mc_var_int(self.id);
        buf.set_mc_u8(self.action);
    }

    fn decode(buf: &mut BytesMut) -> Result<Self> where Self: Sized {
        let id = buf.get_mc_var_int()?;
        let action = buf.get_mc_u8()?;
        return Ok(AnimatePacket { id, action });
    }
}
