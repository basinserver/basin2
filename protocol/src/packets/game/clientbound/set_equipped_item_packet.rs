
use crate::packet::*;
use crate::network::*;
use bytes::BytesMut;
use uuid::Uuid;
use crate::result::*;

pub struct SetEquippedItemPacket {
    pub entity: i32,
    pub slot: EquipmentSlot,
    pub itemStack: ItemStack,
}

impl CodablePacket for SetEquippedItemPacket {
    fn encode(self, buf: &mut BytesMut) {
        buf.set_mc_var_int(self.entity);
        buf.set_mc_var_int(self.slot);
        buf.set_mc_item_stack(self.itemStack);
    }

    fn decode(buf: &mut BytesMut) -> Result<Self> where Self: Sized {
        let entity = buf.get_mc_var_int()?;
        // TODO: UNKNOWN: this.slot = (EquipmentSlot)var1.readEnum(EquipmentSlot.class);
        let itemStack = buf.get_mc_item_stack()?;
        return Ok(SetEquippedItemPacket { entity, slot, itemStack });
    }
}
