use crate::network::*;
use crate::packet::*;
use crate::result::*;
use bytes::BytesMut;

#[derive(PartialEq, Clone, Debug)]
pub struct UseItemOnPacket {
    pub blockHit: BlockHitResult,
    pub hand: InteractionHand,
}

impl CodablePacket for UseItemOnPacket {
    fn encode(self, buf: &mut BytesMut) {
        buf.set_mc_var_int(self.hand as i32);
        buf.set_mc_block_hit_result(self.blockHit);
    }

    fn decode(buf: &mut BytesMut) -> Result<Self>
    where
        Self: Sized,
    {
        let hand: InteractionHand = buf.get_mc_enum()?;
        let blockHit = buf.get_mc_block_hit_result()?;
        return Ok(UseItemOnPacket { blockHit, hand });
    }
}
