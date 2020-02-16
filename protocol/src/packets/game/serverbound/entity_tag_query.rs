use crate::network::*;
use crate::packet::*;
use crate::result::*;
use bytes::BytesMut;

#[derive(PartialEq, Clone, Debug)]
pub struct EntityTagQuery {
    pub transactionId: i32,
    pub entityId: i32,
}

impl CodablePacket for EntityTagQuery {
    fn encode(self, buf: &mut BytesMut) {
        buf.set_mc_var_int(self.transactionId);
        buf.set_mc_var_int(self.entityId);
    }

    fn decode(buf: &mut BytesMut) -> Result<Self>
    where
        Self: Sized,
    {
        let transactionId = buf.get_mc_var_int()?;
        let entityId = buf.get_mc_var_int()?;
        return Ok(EntityTagQuery {
            transactionId,
            entityId,
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::packet::test::*;

    #[test]
    fn test_cycle() -> Result<()> {
        cycle(EntityTagQuery {
            transactionId: 12345,
            entityId: 4321,
        })
    }
}