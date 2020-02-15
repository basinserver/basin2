use crate::packet::*;
use crate::result::*;
use bytes::BytesMut;

pub struct StatusRequestPacket {}

impl CodablePacket for StatusRequestPacket {
    fn encode(self, buf: &mut BytesMut) {}

    fn decode(buf: &mut BytesMut) -> Result<Self>
    where
        Self: Sized,
    {
        return Ok(StatusRequestPacket {});
    }
}
