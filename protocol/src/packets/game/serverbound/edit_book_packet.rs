
use crate::packet::*;
use crate::network::*;
use bytes::BytesMut;
use uuid::Uuid;
use crate::result::*;

pub struct EditBookPacket {
    pub book: ItemStack,
    pub signing: bool,
    pub hand: InteractionHand,
}

impl CodablePacket for EditBookPacket {
    fn encode(self, buf: &mut BytesMut) {
        buf.set_mc_item_stack(self.book);
        buf.set_mc_bool(self.signing);
        buf.set_mc_var_int(self.hand);
    }

    fn decode(buf: &mut BytesMut) -> Result<Self> where Self: Sized {
        let book = buf.get_mc_item_stack()?;
        let signing = buf.get_mc_bool()?;
        // TODO: UNKNOWN: this.hand = (InteractionHand)var1.readEnum(InteractionHand.class);
        return Ok(EditBookPacket { book, signing, hand });
    }
}
