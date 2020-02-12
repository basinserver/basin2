
use crate::packet::*;
use crate::network::*;
use bytes::BytesMut;
use uuid::Uuid;
use crate::result::*;

pub struct OpenBookPacket {
    pub hand: InteractionHand,
}

impl CodablePacket for OpenBookPacket {
    fn encode(self, buf: &mut BytesMut) {
        buf.set_mc_var_int(self.hand);
    }

    fn decode(buf: &mut BytesMut) -> Result<Self> where Self: Sized {
        // TODO: UNKNOWN: this.hand = (InteractionHand)var1.readEnum(InteractionHand.class);
        return Ok(OpenBookPacket { hand });
    }
}
