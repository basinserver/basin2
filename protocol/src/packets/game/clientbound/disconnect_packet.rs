use crate::network::*;
use crate::packet::*;
use basin2_lib::result::*;
use bytes::BytesMut;

#[derive(PartialEq, Clone, Debug)]
pub struct DisconnectPacket {
    pub reason: ChatComponent,
}

impl CodablePacket for DisconnectPacket {
    fn encode(self, buf: &mut BytesMut) {
        buf.set_mc_chat_component(self.reason);
    }

    fn decode(buf: &mut BytesMut) -> Result<Self>
    where
        Self: Sized,
    {
        let reason = buf.get_mc_chat_component()?;
        return Ok(DisconnectPacket { reason });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::packet::test::*;

    #[test]
    fn test_cycle() -> Result<()> {
        cycle(DisconnectPacket {
            reason: ChatComponent::from("disconnect".to_string()),
        })
    }
}
