use crate::network::*;
use crate::packet::*;
use basin2_lib::result::*;
use bytes::BytesMut;

#[derive(PartialEq, Clone, Debug)]
pub struct ChatPacket {
    pub message: ChatComponent,
    pub chat_type: ChatType,
}

impl CodablePacket for ChatPacket {
    fn encode(self, buf: &mut BytesMut) {
        buf.set_mc_chat_component(self.message);
        buf.set_mc_u8(self.chat_type as u8);
    }

    fn decode(buf: &mut BytesMut) -> Result<Self>
    where
        Self: Sized,
    {
        let message = buf.get_mc_chat_component()?;
        let chat_type: ChatType = buf.get_mc_enum_u8()?;
        return Ok(ChatPacket { message, chat_type });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::packet::test::*;

    #[test]
    fn test_cycle() -> Result<()> {
        cycle(ChatPacket {
            message: ChatComponent::from("chat component".to_string()),
            chat_type: ChatType::Chat,
        })
    }
}
