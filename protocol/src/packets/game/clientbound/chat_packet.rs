
use crate::packet::*;
use crate::network::*;
use bytes::BytesMut;
use uuid::Uuid;
use crate::result::*;

pub struct ChatPacket {
    pub message: ChatComponent,
    pub type: ChatType,
}

impl CodablePacket for ChatPacket {
    fn encode(self, buf: &mut BytesMut) {
        buf.set_mc_chat_component(self.message);
        // TODO: UNKNOWN: var1.writeByte(this.type.getIndex());
    }

    fn decode(buf: &mut BytesMut) -> Result<Self> where Self: Sized {
        let message = buf.get_mc_chat_component()?;
        // TODO: UNKNOWN: this.type = ChatType.getForIndex(var1.readByte());
        return Ok(ChatPacket { message, type });
    }
}
