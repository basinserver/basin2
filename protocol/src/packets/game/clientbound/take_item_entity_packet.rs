use crate::network::*;
use crate::packet::*;
use basin2_lib::result::*;
use bytes::BytesMut;

#[derive(PartialEq, Clone, Debug)]
pub struct TakeItemEntityPacket {
    pub itemId: i32,
    pub playerId: i32,
    pub amount: i32,
}

impl CodablePacket for TakeItemEntityPacket {
    fn encode(self, buf: &mut BytesMut) {
        buf.set_mc_var_int(self.itemId);
        buf.set_mc_var_int(self.playerId);
        buf.set_mc_var_int(self.amount);
    }

    fn decode(buf: &mut BytesMut) -> Result<Self>
    where
        Self: Sized,
    {
        let itemId = buf.get_mc_var_int()?;
        let playerId = buf.get_mc_var_int()?;
        let amount = buf.get_mc_var_int()?;
        return Ok(TakeItemEntityPacket {
            itemId,
            playerId,
            amount,
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::packet::test::*;

    #[test]
    fn test_cycle() -> Result<()> {
        cycle(TakeItemEntityPacket {
            itemId: 234456,
            playerId: 5673,
            amount: 12,
        })
    }
}
