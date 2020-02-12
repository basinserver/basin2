
use crate::packet::*;
use crate::network::*;
use bytes::BytesMut;
use uuid::Uuid;
use crate::result::*;

pub struct ContainerClickPacket {
    pub containerId: i32,
    pub slotNum: i32,
    pub buttonNum: i32,
    pub uid: i16,
    pub itemStack: ItemStack,
    pub clickType: ClickType,
}

impl CodablePacket for ContainerClickPacket {
    fn encode(self, buf: &mut BytesMut) {
        buf.set_mc_u8(self.containerId);
        buf.set_mc_i16(self.slotNum);
        buf.set_mc_u8(self.buttonNum);
        buf.set_mc_i16(self.uid);
        buf.set_mc_var_int(self.clickType);
        buf.set_mc_item_stack(self.itemStack);
    }

    fn decode(buf: &mut BytesMut) -> Result<Self> where Self: Sized {
        let containerId = buf.get_mc_u8()?;
        let slotNum = buf.get_mc_i16()?;
        let buttonNum = buf.get_mc_u8()?;
        let uid = buf.get_mc_i16()?;
        // TODO: UNKNOWN: this.clickType = (ClickType)var1.readEnum(ClickType.class);
        let itemStack = buf.get_mc_item_stack()?;
        return Ok(ContainerClickPacket { containerId, slotNum, buttonNum, uid, itemStack, clickType });
    }
}
