use crate::result::*;
use bytes::BytesMut;

pub trait CodablePacket {
    fn encode(self, buf: &mut BytesMut);
    fn decode(buf: &mut BytesMut) -> Result<Self>
    where
        Self: Sized;
}

pub trait PacketContainer {
    fn encode(self, buf: &mut BytesMut);
    fn decode(id: i32, buf: &mut BytesMut) -> Result<Self>
    where
        Self: Sized;
}
