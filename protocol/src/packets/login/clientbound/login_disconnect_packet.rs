
use crate::packet::*;
use crate::network::*;
use bytes::BytesMut;
use uuid::Uuid;
use crate::result::*;

pub struct LoginDisconnectPacket {
    pub reason: ChatComponent,
}

impl CodablePacket for LoginDisconnectPacket {
    fn encode(self, buf: &mut BytesMut) {
        buf.set_mc_chat_component(self.reason);
    }

    fn decode(buf: &mut BytesMut) -> Result<Self> where Self: Sized {
        // TODO: UNKNOWN: this.reason = Component.Serializer.fromJsonLenient(var1.readUtf(262144));
        return Ok(LoginDisconnectPacket { reason });
    }
}
