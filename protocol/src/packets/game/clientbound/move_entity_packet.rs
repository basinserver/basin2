use crate::network::*;
use crate::packet::*;
use crate::result::*;
use bytes::BytesMut;

#[derive(PartialEq, Clone, Debug)]
pub struct MoveEntityPacket {
    pub entityId: i32,
}

impl CodablePacket for MoveEntityPacket {
    fn encode(self, buf: &mut BytesMut) {
        buf.set_mc_var_int(self.entityId);
    }

    fn decode(buf: &mut BytesMut) -> Result<Self>
    where
        Self: Sized,
    {
        let entityId = buf.get_mc_var_int()?;
        return Ok(MoveEntityPacket { entityId });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::packet::test::*;

    #[test]
    fn test_cycle() -> Result<()> {
        cycle(MoveEntityPacket {
            entityId: 234,
        })
    }
}