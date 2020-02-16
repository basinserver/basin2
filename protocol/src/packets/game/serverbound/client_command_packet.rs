use crate::network::*;
use crate::packet::*;
use crate::result::*;
use bytes::BytesMut;

#[derive(PartialEq, Clone, Debug)]
pub struct ClientCommandPacket {
    pub action: ClientCommandPacketAction,
}

impl CodablePacket for ClientCommandPacket {
    fn encode(self, buf: &mut BytesMut) {
        buf.set_mc_var_int(self.action as i32);
    }

    fn decode(buf: &mut BytesMut) -> Result<Self>
    where
        Self: Sized,
    {
        let action: ClientCommandPacketAction = buf.get_mc_enum()?;
        return Ok(ClientCommandPacket { action });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::packet::test::*;

    #[test]
    fn test_cycle() -> Result<()> {
        cycle(ClientCommandPacket {
            action: ClientCommandPacketAction::PerformRespawn,
        })
    }
}
