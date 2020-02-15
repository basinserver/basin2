
use crate::packet::*;
use crate::network::*;
use bytes::BytesMut;
use crate::result::*;

pub struct SetCarriedItemPacket {
    pub slot: u8,
}

impl CodablePacket for SetCarriedItemPacket {
    fn encode(self, buf: &mut BytesMut) {
        buf.set_mc_u8(self.slot);
    }

    fn decode(buf: &mut BytesMut) -> Result<Self> where Self: Sized {
        let slot = buf.get_mc_u8()?;
        return Ok(SetCarriedItemPacket { slot });
    }
}
