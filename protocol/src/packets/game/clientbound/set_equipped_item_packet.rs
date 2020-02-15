
use crate::packet::*;
use crate::network::*;
use bytes::BytesMut;
use crate::result::*;

pub struct SetEquippedItemPacket {
    pub entity: i32,
    pub slot: EquipmentSlot,
    pub itemStack: ItemStack,
}

impl CodablePacket for SetEquippedItemPacket {
    fn encode(self, buf: &mut BytesMut) {
        buf.set_mc_var_int(self.entity);
        buf.set_mc_var_int(self.slot as i32);
        buf.set_mc_item_stack(self.itemStack);
    }

    fn decode(buf: &mut BytesMut) -> Result<Self> where Self: Sized {
        let entity = buf.get_mc_var_int()?;
        let slot: EquipmentSlot = buf.get_mc_enum()?;
        let itemStack = buf.get_mc_item_stack()?;
        return Ok(SetEquippedItemPacket { entity, slot, itemStack });
    }
}
