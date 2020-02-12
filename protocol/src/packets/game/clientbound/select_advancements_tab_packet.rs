
use crate::packet::*;
use crate::network::*;
use bytes::BytesMut;
use uuid::Uuid;
use crate::result::*;

pub struct SelectAdvancementsTabPacket {
    pub tab: ResourceLocation,
}

impl CodablePacket for SelectAdvancementsTabPacket {
    fn encode(self, buf: &mut BytesMut) {
        /* TODO: NOT FOUND */
    }

    fn decode(buf: &mut BytesMut) -> Result<Self> where Self: Sized {
        /* TODO: NOT FOUND */
        return Ok(SelectAdvancementsTabPacket { tab });
    }
}
