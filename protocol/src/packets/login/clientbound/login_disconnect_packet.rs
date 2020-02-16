use crate::network::*;
use crate::packet::*;
use crate::result::*;
use bytes::BytesMut;

#[derive(PartialEq, Clone, Debug)]
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::packet::test::*;

    #[test]
    fn test_cycle() -> Result<()> {
        cycle(LoginDisconnectPacket {
            reason: "test".to_string(),
        })
    }
}
