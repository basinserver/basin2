
use crate::packet::*;
use crate::network::*;
use bytes::BytesMut;
use crate::result::*;

pub struct RemoveMobEffectPacket {
    pub entityId: i32,
    pub effect: MobEffect,
}

impl CodablePacket for RemoveMobEffectPacket {
    fn encode(self, buf: &mut BytesMut) {
        buf.set_mc_var_int(self.entityId);
        buf.set_mc_u8(self.effect as u8);
    }

    fn decode(buf: &mut BytesMut) -> Result<Self> where Self: Sized {
        let entityId = buf.get_mc_var_int()?;
        let effect = buf.get_mc_enum_u8()?;
        return Ok(RemoveMobEffectPacket { entityId, effect });
    }
}
