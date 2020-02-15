
use crate::packet::*;
use crate::network::*;
use bytes::BytesMut;
use crate::result::*;

pub struct OpenBookPacket {
    pub hand: InteractionHand,
}

impl CodablePacket for OpenBookPacket {
    fn encode(self, buf: &mut BytesMut) {
        buf.set_mc_var_int(self.hand as i32);
    }

    fn decode(buf: &mut BytesMut) -> Result<Self> where Self: Sized {
        let hand: InteractionHand = buf.get_mc_enum()?;
        return Ok(OpenBookPacket { hand });
    }
}
