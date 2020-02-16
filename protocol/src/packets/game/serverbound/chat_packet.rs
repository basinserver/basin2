use crate::network::*;
use crate::packet::*;
use crate::result::*;
use bytes::BytesMut;

#[derive(PartialEq, Clone, Debug)]
pub struct ChatPacket {
    pub message: String,
}

impl CodablePacket for ChatPacket {
    fn encode(self, buf: &mut BytesMut) {
        buf.set_mc_string(self.message);
    }

    fn decode(buf: &mut BytesMut) -> Result<Self>
    where
        Self: Sized,
    {
        let message = buf.get_mc_string(256)?;
        return Ok(ChatPacket { message });
    }
}
