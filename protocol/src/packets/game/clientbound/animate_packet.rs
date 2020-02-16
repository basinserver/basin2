use crate::network::*;
use crate::packet::*;
use crate::result::*;
use bytes::BytesMut;

#[derive(PartialEq, Clone, Debug)]
pub struct AnimatePacket {
    pub id: i32,
    pub action: u8,
}

impl CodablePacket for AnimatePacket {
    fn encode(self, buf: &mut BytesMut) {
        buf.set_mc_var_int(self.id);
        buf.set_mc_u8(self.action);
    }

    fn decode(buf: &mut BytesMut) -> Result<Self>
    where
        Self: Sized,
    {
        let id = buf.get_mc_var_int()?;
        let action = buf.get_mc_u8()?;
        return Ok(AnimatePacket { id, action });
    }
}
