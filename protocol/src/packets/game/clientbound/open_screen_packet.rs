
use crate::packet::*;
use crate::network::*;
use bytes::BytesMut;
use uuid::Uuid;
use crate::result::*;

pub struct OpenScreenPacket {
    pub containerId: i32,
    pub type: i32,
    pub title: ChatComponent,
}

impl CodablePacket for OpenScreenPacket {
    fn encode(self, buf: &mut BytesMut) {
        buf.set_mc_var_int(self.containerId);
        buf.set_mc_var_int(self.type);
        buf.set_mc_chat_component(self.title);
    }

    fn decode(buf: &mut BytesMut) -> Result<Self> where Self: Sized {
        let containerId = buf.get_mc_var_int()?;
        let type = buf.get_mc_var_int()?;
        let title = buf.get_mc_chat_component()?;
        return Ok(OpenScreenPacket { containerId, type, title });
    }
}
