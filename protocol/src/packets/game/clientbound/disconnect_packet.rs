
use crate::packet::*;
use crate::network::*;
use bytes::BytesMut;
use crate::result::*;

pub struct DisconnectPacket {
    pub reason: ChatComponent,
}

impl CodablePacket for DisconnectPacket {
    fn encode(self, buf: &mut BytesMut) {
        buf.set_mc_chat_component(self.reason);
    }

    fn decode(buf: &mut BytesMut) -> Result<Self> where Self: Sized {
        let reason = buf.get_mc_chat_component()?;
        return Ok(DisconnectPacket { reason });
    }
}
