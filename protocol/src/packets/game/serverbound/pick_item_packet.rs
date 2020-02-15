use crate::network::*;
use crate::packet::*;
use crate::result::*;
use bytes::BytesMut;

pub struct PickItemPacket {
    pub slot: i32,
}

impl CodablePacket for PickItemPacket {
    fn encode(self, buf: &mut BytesMut) {
        buf.set_mc_var_int(self.slot);
    }

    fn decode(buf: &mut BytesMut) -> Result<Self>
    where
        Self: Sized,
    {
        let slot = buf.get_mc_var_int()?;
        return Ok(PickItemPacket { slot });
    }
}
