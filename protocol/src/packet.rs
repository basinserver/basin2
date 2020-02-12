use bytes::BytesMut;
use crate::result::*;

pub trait CodablePacket {
    fn encode(self, buf: &mut BytesMut);
    fn decode(buf: &mut BytesMut) -> Result<Self> where Self: Sized;
}
