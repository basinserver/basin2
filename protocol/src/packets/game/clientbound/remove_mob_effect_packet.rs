
use crate::packet::*;
use crate::network::*;
use bytes::BytesMut;
use uuid::Uuid;
use crate::result::*;

pub struct RemoveMobEffectPacket {
    pub entityId: i32,
    pub effect: MobEffect,
}

impl CodablePacket for RemoveMobEffectPacket {
    fn encode(self, buf: &mut BytesMut) {
        buf.set_mc_var_int(self.entityId);
        // TODO: UNKNOWN: var1.writeByte(MobEffect.getId(this.effect));
    }

    fn decode(buf: &mut BytesMut) -> Result<Self> where Self: Sized {
        let entityId = buf.get_mc_var_int()?;
        // TODO: UNKNOWN: this.effect = MobEffect.byId(var1.readUnsignedByte());
        return Ok(RemoveMobEffectPacket { entityId, effect });
    }
}
