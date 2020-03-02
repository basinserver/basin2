use crate::network::*;
use crate::packet::*;
use basin2_lib::result::*;
use bytes::BytesMut;

#[derive(PartialEq, Clone, Debug)]
pub struct StatusResponsePacket {
    pub status: ServerStatus,
}

impl CodablePacket for StatusResponsePacket {
    fn encode(self, buf: &mut BytesMut) {
        buf.set_mc_string(serde_json::to_string(&self.status).unwrap());
    }

    fn decode(buf: &mut BytesMut) -> Result<Self>
    where
        Self: Sized,
    {
        let status: ServerStatus = serde_json::from_str(&*buf.get_mc_string(32767)?)?;
        return Ok(StatusResponsePacket { status });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::packet::test::*;

    #[test]
    fn test_cycle() -> Result<()> {
        cycle(StatusResponsePacket {
            status: ServerStatus {
                description: ChatComponent::from("a testing system".to_string()).serialize(),
                players: ServerStatusPlayers {
                    max: 20,
                    online: 5,
                    sample: Some(vec![GameProfile {
                        uuid: None,
                        name: "testName".to_string(),
                        legacy: false,
                    }]),
                },
                version: ServerStatusVersion {
                    name: "a.b.c".to_string(),
                    protocol: 1234,
                },
                favicon: None,
            },
        })
    }
}
