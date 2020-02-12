
use crate::packet::*;
use crate::network::*;
use bytes::BytesMut;
use uuid::Uuid;
use crate::result::*;

pub struct ContainerSetContentPacket {
    pub containerId: i32,
    pub items: undefined,
}

impl CodablePacket for ContainerSetContentPacket {
    fn encode(self, buf: &mut BytesMut) {
        /* TODO: NOT FOUND */
    }

    fn decode(buf: &mut BytesMut) -> Result<Self> where Self: Sized {
        /* TODO: NOT FOUND */
        return Ok(ContainerSetContentPacket { containerId, items });
    }
}
