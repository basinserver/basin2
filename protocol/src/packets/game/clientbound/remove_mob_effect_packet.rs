use crate::network::*;
use crate::packet::*;
use crate::result::*;
use bytes::BytesMut;

#[derive(PartialEq, Clone, Debug)]
pub struct RemoveMobEffectPacket {
    pub entityId: i32,
    pub effect: MobEffect,
}

impl CodablePacket for RemoveMobEffectPacket {
    fn encode(self, buf: &mut BytesMut) {
        buf.set_mc_var_int(self.entityId);
        buf.set_mc_u8(self.effect as u8);
    }

    fn decode(buf: &mut BytesMut) -> Result<Self>
    where
        Self: Sized,
    {
        let entityId = buf.get_mc_var_int()?;
        let effect = buf.get_mc_enum_u8()?;
        return Ok(RemoveMobEffectPacket { entityId, effect });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::packet::test::*;

    #[test]
    fn test_cycle() -> Result<()> {
        cycle(RemoveMobEffectPacket {
            entityId: 56435,
            effect: 56,
        })
    }
}
