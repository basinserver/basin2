use crate::network::*;
use crate::packet::*;
use crate::result::*;
use bytes::BytesMut;

#[derive(PartialEq, Clone, Debug)]
pub struct EntityEventPacket {
    pub entityId: i32,
    pub eventId: u8,
}

impl CodablePacket for EntityEventPacket {
    fn encode(self, buf: &mut BytesMut) {
        buf.set_mc_i32(self.entityId);
        buf.set_mc_u8(self.eventId);
    }

    fn decode(buf: &mut BytesMut) -> Result<Self>
    where
        Self: Sized,
    {
        let entityId = buf.get_mc_i32()?;
        let eventId = buf.get_mc_u8()?;
        return Ok(EntityEventPacket { entityId, eventId });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::packet::test::*;

    #[test]
    fn test_cycle() -> Result<()> {
        cycle(EntityEventPacket {
            entityId: 5423432,
            eventId: 12,
        })
    }
}
