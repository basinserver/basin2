use crate::network::*;
use crate::packet::*;
use crate::result::*;
use bytes::BytesMut;

pub struct SetCreativeModeSlotPacket {
    pub slotNum: i16,
    pub itemStack: ItemStack,
}

impl CodablePacket for SetCreativeModeSlotPacket {
    fn encode(self, buf: &mut BytesMut) {
        buf.set_mc_i16(self.slotNum);
        buf.set_mc_item_stack(self.itemStack);
    }

    fn decode(buf: &mut BytesMut) -> Result<Self>
    where
        Self: Sized,
    {
        let slotNum = buf.get_mc_i16()?;
        let itemStack = buf.get_mc_item_stack()?;
        return Ok(SetCreativeModeSlotPacket { slotNum, itemStack });
    }
}
