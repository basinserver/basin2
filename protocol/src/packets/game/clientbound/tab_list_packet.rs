use crate::network::*;
use crate::packet::*;
use crate::result::*;
use bytes::BytesMut;

pub struct TabListPacket {
    pub header: ChatComponent,
    pub footer: ChatComponent,
}

impl CodablePacket for TabListPacket {
    fn encode(self, buf: &mut BytesMut) {
        buf.set_mc_chat_component(self.header);
        buf.set_mc_chat_component(self.footer);
    }

    fn decode(buf: &mut BytesMut) -> Result<Self>
    where
        Self: Sized,
    {
        let header = buf.get_mc_chat_component()?;
        let footer = buf.get_mc_chat_component()?;
        return Ok(TabListPacket { header, footer });
    }
}
