use crate::network::*;
use crate::packet::*;
use basin2_lib::result::*;
use bytes::BytesMut;

#[derive(PartialEq, Clone, Debug)]
pub struct EditBookPacket {
    pub book: ItemStack,
    pub signing: bool,
    pub hand: InteractionHand,
}

impl CodablePacket for EditBookPacket {
    fn encode(self, buf: &mut BytesMut) {
        buf.set_mc_item_stack(self.book);
        buf.set_mc_bool(self.signing);
        buf.set_mc_var_int(self.hand as i32);
    }

    fn decode(buf: &mut BytesMut) -> Result<Self>
    where
        Self: Sized,
    {
        let book = buf.get_mc_item_stack()?;
        let signing = buf.get_mc_bool()?;
        let hand: InteractionHand = buf.get_mc_enum()?;
        return Ok(EditBookPacket {
            book,
            signing,
            hand,
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::packet::test::*;

    #[test]
    fn test_cycle() -> Result<()> {
        cycle(EditBookPacket {
            book: ItemStack::empty(),
            signing: true,
            hand: InteractionHand::MainHand,
        })
    }
}
