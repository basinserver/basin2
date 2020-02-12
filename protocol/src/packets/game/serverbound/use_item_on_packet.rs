
use crate::packet::*;
use crate::network::*;
use bytes::BytesMut;
use uuid::Uuid;
use crate::result::*;

pub struct UseItemOnPacket {
    pub blockHit: BlockHitResult,
    pub hand: InteractionHand,
}

impl CodablePacket for UseItemOnPacket {
    fn encode(self, buf: &mut BytesMut) {
        buf.set_mc_var_int(self.hand);
        buf.set_mc_block_hit_result(self.blockHit);
    }

    fn decode(buf: &mut BytesMut) -> Result<Self> where Self: Sized {
        // TODO: UNKNOWN: this.hand = (InteractionHand)var1.readEnum(InteractionHand.class);
        let blockHit = buf.get_mc_block_hit_result()?;
        return Ok(UseItemOnPacket { blockHit, hand });
    }
}
