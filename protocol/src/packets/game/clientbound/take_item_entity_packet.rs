use crate::network::*;
use crate::packet::*;
use crate::result::*;
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
