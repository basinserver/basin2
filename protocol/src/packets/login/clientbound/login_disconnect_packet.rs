use crate::network::*;
use crate::packet::*;
use crate::result::*;
use bytes::BytesMut;

pub struct LoginDisconnectPacket {
    pub reason: ChatComponent,
}

impl CodablePacket for LoginDisconnectPacket {
    fn encode(self, buf: &mut BytesMut) {
        buf.set_mc_chat_component(self.reason);
    }

    fn decode(buf: &mut BytesMut) -> Result<Self>
    where
        Self: Sized,
    {
        let reason = buf.get_mc_string(262144)?;
        return Ok(LoginDisconnectPacket { reason });
    }
}
