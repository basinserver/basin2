
use crate::packet::*;
use crate::network::*;
use bytes::BytesMut;
use uuid::Uuid;
use crate::result::*;

pub struct RemoveEntitiesPacket {
    pub entityIds: Vec<i32>,
}

impl CodablePacket for RemoveEntitiesPacket {
    fn encode(self, buf: &mut BytesMut) {
        /* TODO: NOT FOUND */
    }

    fn decode(buf: &mut BytesMut) -> Result<Self> where Self: Sized {
        /* TODO: NOT FOUND */
        return Ok(RemoveEntitiesPacket { entityIds });
    }
}
