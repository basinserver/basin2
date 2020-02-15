use crate::network::*;
use crate::packet::*;
use crate::result::*;
use bytes::BytesMut;

pub struct UpdateAttributesPacket {
    pub entityId: i32,
    pub attributes: Vec<EntityAttribute>,
}

impl CodablePacket for UpdateAttributesPacket {
    fn encode(self, buf: &mut BytesMut) {
        buf.set_mc_var_int(self.entityId);
        buf.set_mc_var_int(self.attributes.len() as i32);
        for attribute in self.attributes {
            buf.set_mc_string(attribute.name);
            buf.set_mc_f64(attribute.base);
            buf.set_mc_var_int(attribute.modifiers.len() as i32);
            for modifier in attribute.modifiers {
                buf.set_mc_uuid(modifier.uuid);
                buf.set_mc_f64(modifier.amount);
                buf.set_mc_u8(modifier.operation as u8);
            }
        }
    }

    fn decode(buf: &mut BytesMut) -> Result<Self>
    where
        Self: Sized,
    {
        let entityId = buf.get_mc_var_int()?;
        let attribute_count = buf.get_mc_var_int()?;
        let mut attributes: Vec<EntityAttribute> = vec![];
        for _ in 0..attribute_count {
            let name = buf.get_mc_string(64)?;
            let base = buf.get_mc_f64()?;
            let modifier_count = buf.get_mc_var_int()?;
            let mut modifiers: Vec<EntityAttributeModifier> = vec![];
            for _ in 0..modifier_count {
                let uuid = buf.get_mc_uuid()?;
                let amount = buf.get_mc_f64()?;
                let operation: EntityAttributeModifierOperation = buf.get_mc_enum_u8()?;
                modifiers.push(EntityAttributeModifier {
                    uuid,
                    amount,
                    operation,
                });
            }
            attributes.push(EntityAttribute {
                name,
                base,
                modifiers,
            });
        }
        return Ok(UpdateAttributesPacket {
            entityId,
            attributes,
        });
    }
}
