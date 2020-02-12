
use crate::packet::*;
use crate::network::*;
use bytes::BytesMut;
use uuid::Uuid;
use crate::result::*;

pub struct SetTitlesPacket {
    pub type: SetTitlesPacketType,
    pub text: ChatComponent,
    pub fadeInTime: i32,
    pub stayTime: i32,
    pub fadeOutTime: i32,
}

impl CodablePacket for SetTitlesPacket {
    fn encode(self, buf: &mut BytesMut) {
        /* TODO: NOT FOUND */
    }

    fn decode(buf: &mut BytesMut) -> Result<Self> where Self: Sized {
        /* TODO: NOT FOUND */
        return Ok(SetTitlesPacket { type, text, fadeInTime, stayTime, fadeOutTime });
    }
}
