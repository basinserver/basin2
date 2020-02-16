use crate::network::*;
use crate::packet::*;
use crate::result::*;
use bytes::BytesMut;

#[derive(PartialEq, Clone, Debug)]
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

    fn decode(buf: &mut BytesMut) -> Result<Self>
    where
        Self: Sized,
    {
        let entity = buf.get_mc_var_int()?;
        let slot: EquipmentSlot = buf.get_mc_enum()?;
        let itemStack = buf.get_mc_item_stack()?;
        return Ok(SetEquippedItemPacket {
            entity,
            slot,
            itemStack,
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::packet::test::*;

    #[test]
    fn test_cycle() -> Result<()> {
        cycle(SetEquippedItemPacket {
            entity: 456456,
            slot: EquipmentSlot::Legs,
            itemStack: ItemStack::empty(),
        })
    }
}