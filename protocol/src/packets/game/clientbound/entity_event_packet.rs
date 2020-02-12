
use crate::packet::*;
use crate::network::*;
use bytes::BytesMut;
use uuid::Uuid;
use crate::result::*;

pub struct EntityEventPacket {
    pub entityId: i32,
    pub eventId: u8,
}

impl CodablePacket for EntityEventPacket {
    fn encode(self, buf: &mut BytesMut) {
        buf.set_mc_i32(self.entityId);
        buf.set_mc_u8(self.eventId);
    }

    fn decode(buf: &mut BytesMut) -> Result<Self> where Self: Sized {
        let entityId = buf.get_mc_i32()?;
        let eventId = buf.get_mc_u8()?;
        return Ok(EntityEventPacket { entityId, eventId });
    }
}
