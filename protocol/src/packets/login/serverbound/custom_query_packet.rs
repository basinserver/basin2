
use crate::packet::*;
use crate::network::*;
use bytes::BytesMut;
use uuid::Uuid;
use crate::result::*;

pub struct CustomQueryPacket {
    pub transactionId: i32,
    pub data: BytesMut,
}

impl CodablePacket for CustomQueryPacket {
    fn encode(self, buf: &mut BytesMut) {
        /* TODO: NOT FOUND */
    }

    fn decode(buf: &mut BytesMut) -> Result<Self> where Self: Sized {
        /* TODO: NOT FOUND */
        return Ok(CustomQueryPacket { transactionId, data });
    }
}
