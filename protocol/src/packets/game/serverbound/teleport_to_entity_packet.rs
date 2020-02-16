use crate::network::*;
use crate::packet::*;
use crate::result::*;
use bytes::BytesMut;
use uuid::Uuid;

#[derive(PartialEq, Clone, Debug)]
pub struct TeleportToEntityPacket {
    pub uuid: Uuid,
}

impl CodablePacket for TeleportToEntityPacket {
    fn encode(self, buf: &mut BytesMut) {
        buf.set_mc_uuid(self.uuid);
    }

    fn decode(buf: &mut BytesMut) -> Result<Self>
    where
        Self: Sized,
    {
        let uuid = buf.get_mc_uuid()?;
        return Ok(TeleportToEntityPacket { uuid });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::packet::test::*;

    #[test]
    fn test_cycle() -> Result<()> {
        cycle(TeleportToEntityPacket {
            uuid: Uuid::new_v4(),
        })
    }
}
