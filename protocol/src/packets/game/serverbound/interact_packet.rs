
use crate::packet::*;
use crate::network::*;
use bytes::BytesMut;
use uuid::Uuid;
use crate::result::*;

pub struct InteractPacket {
    pub entityId: i32,
    pub action: InteractPacketAction,
    pub hand: InteractionHand,
}

impl CodablePacket for InteractPacket {
    fn encode(self, buf: &mut BytesMut) {
        /* TODO: NOT FOUND */
    }

    fn decode(buf: &mut BytesMut) -> Result<Self> where Self: Sized {
        /* TODO: NOT FOUND */
        return Ok(InteractPacket { entityId, action, hand });
    }
}
