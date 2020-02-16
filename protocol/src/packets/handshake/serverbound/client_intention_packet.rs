use crate::network::*;
use crate::packet::*;
use crate::result::*;
use bytes::BytesMut;

#[derive(PartialEq, Clone, Debug)]
pub struct ClientIntentionPacket {
    pub protocolVersion: i32,
    pub hostName: String,
    pub port: u16,
    pub intention: ConnectionProtocol,
}

impl CodablePacket for ClientIntentionPacket {
    fn encode(self, buf: &mut BytesMut) {
        buf.set_mc_var_int(self.protocolVersion);
        buf.set_mc_string(self.hostName);
        buf.set_mc_u16(self.port);
        buf.set_mc_var_int(self.intention as i32);
    }

    fn decode(buf: &mut BytesMut) -> Result<Self>
    where
        Self: Sized,
    {
        let protocolVersion = buf.get_mc_var_int()?;
        let hostName = buf.get_mc_string(255)?;
        let port = buf.get_mc_u16()?;
        let intention: ConnectionProtocol = buf.get_mc_enum()?;
        return Ok(ClientIntentionPacket {
            protocolVersion,
            hostName,
            port,
            intention,
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::packet::test::*;

    #[test]
    fn test_cycle() -> Result<()> {
        cycle(ClientIntentionPacket {
            protocolVersion: 1234,
            hostName: "testHost".to_string(),
            port: 54321,
            intention: ConnectionProtocol::Game,
        })
    }
}
