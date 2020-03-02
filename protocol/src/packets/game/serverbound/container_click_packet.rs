use crate::network::*;
use crate::packet::*;
use basin2_lib::result::*;
use bytes::BytesMut;

#[derive(PartialEq, Clone, Debug)]
pub struct ContainerClickPacket {
    pub containerId: u8,
    pub slotNum: i16,
    pub buttonNum: u8,
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
        buf.set_mc_var_int(self.clickType as i32);
        buf.set_mc_item_stack(self.itemStack);
    }

    fn decode(buf: &mut BytesMut) -> Result<Self>
    where
        Self: Sized,
    {
        let containerId = buf.get_mc_u8()?;
        let slotNum = buf.get_mc_i16()?;
        let buttonNum = buf.get_mc_u8()?;
        let uid = buf.get_mc_i16()?;
        let clickType: ClickType = buf.get_mc_enum()?;
        let itemStack = buf.get_mc_item_stack()?;
        return Ok(ContainerClickPacket {
            containerId,
            slotNum,
            buttonNum,
            uid,
            itemStack,
            clickType,
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::packet::test::*;

    #[test]
    fn test_cycle() -> Result<()> {
        cycle(ContainerClickPacket {
            containerId: 123,
            slotNum: 30,
            buttonNum: 1,
            uid: 26000,
            itemStack: ItemStack::empty(),
            clickType: ClickType::QuickMove,
        })
    }
}
