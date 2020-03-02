use crate::network::*;
use crate::packet::*;
use basin2_lib::result::*;
use bytes::BytesMut;

#[derive(PartialEq, Clone, Debug)]
pub struct UpdateMobEffectPacket {
    pub entityId: i32,
    pub effectId: u8,
    pub effectAmplifier: u8,
    pub effectDurationTicks: i32,
    pub flags: u8,
}

impl CodablePacket for UpdateMobEffectPacket {
    fn encode(self, buf: &mut BytesMut) {
        buf.set_mc_var_int(self.entityId);
        buf.set_mc_u8(self.effectId);
        buf.set_mc_u8(self.effectAmplifier);
        buf.set_mc_var_int(self.effectDurationTicks);
        buf.set_mc_u8(self.flags);
    }

    fn decode(buf: &mut BytesMut) -> Result<Self>
    where
        Self: Sized,
    {
        let entityId = buf.get_mc_var_int()?;
        let effectId = buf.get_mc_u8()?;
        let effectAmplifier = buf.get_mc_u8()?;
        let effectDurationTicks = buf.get_mc_var_int()?;
        let flags = buf.get_mc_u8()?;
        return Ok(UpdateMobEffectPacket {
            entityId,
            effectId,
            effectAmplifier,
            effectDurationTicks,
            flags,
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::packet::test::*;

    #[test]
    fn test_cycle() -> Result<()> {
        cycle(UpdateMobEffectPacket {
            entityId: 4235356,
            effectId: 12,
            effectAmplifier: 2,
            effectDurationTicks: 245435,
            flags: 4,
        })
    }
}
