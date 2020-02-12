
use crate::packet::*;
use crate::network::*;
use bytes::BytesMut;
use uuid::Uuid;
use crate::result::*;

pub struct SeenAdvancementsPacket {
    pub action: SeenAdvancementsPacketAction,
    pub tab: ResourceLocation,
}

impl CodablePacket for SeenAdvancementsPacket {
    fn encode(self, buf: &mut BytesMut) {
        /* TODO: NOT FOUND */
    }

    fn decode(buf: &mut BytesMut) -> Result<Self> where Self: Sized {
        /* TODO: NOT FOUND */
        return Ok(SeenAdvancementsPacket { action, tab });
    }
}
