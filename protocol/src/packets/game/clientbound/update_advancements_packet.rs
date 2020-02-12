
use crate::packet::*;
use crate::network::*;
use bytes::BytesMut;
use uuid::Uuid;
use crate::result::*;

pub struct UpdateAdvancementsPacket {
    pub reset: bool,
    pub added: undefined,
    pub removed: undefined,
    pub progress: undefined,
}

impl CodablePacket for UpdateAdvancementsPacket {
    fn encode(self, buf: &mut BytesMut) {
        /* TODO: NOT FOUND */
    }

    fn decode(buf: &mut BytesMut) -> Result<Self> where Self: Sized {
        /* TODO: NOT FOUND */
        return Ok(UpdateAdvancementsPacket { reset, added, removed, progress });
    }
}
