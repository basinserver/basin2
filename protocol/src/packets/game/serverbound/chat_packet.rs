
use crate::packet::*;
use crate::network::*;
use bytes::BytesMut;
use uuid::Uuid;
use crate::result::*;

pub struct ChatPacket {
    pub message: String,
}

impl CodablePacket for ChatPacket {
    fn encode(self, buf: &mut BytesMut) {
        buf.set_mc_string(self.message);
    }

    fn decode(buf: &mut BytesMut) -> Result<Self> where Self: Sized {
        let message = buf.get_mc_string_bounded(256)?;
        return Ok(ChatPacket { message });
    }
}
