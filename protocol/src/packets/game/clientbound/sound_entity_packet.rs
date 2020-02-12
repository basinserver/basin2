
use crate::packet::*;
use crate::network::*;
use bytes::BytesMut;
use uuid::Uuid;
use crate::result::*;

pub struct SoundEntityPacket {
    pub sound: SoundEvent,
    pub source: SoundSource,
    pub id: i32,
    pub volume: f32,
    pub pitch: f32,
}

impl CodablePacket for SoundEntityPacket {
    fn encode(self, buf: &mut BytesMut) {
        // TODO: UNKNOWN: var1.writeVarInt(Registry.SOUND_EVENT.getId(this.sound));
        buf.set_mc_var_int(self.source);
        buf.set_mc_var_int(self.id);
        buf.set_mc_f32(self.volume);
        buf.set_mc_f32(self.pitch);
    }

    fn decode(buf: &mut BytesMut) -> Result<Self> where Self: Sized {
        // TODO: UNKNOWN: this.sound = (SoundEvent)Registry.SOUND_EVENT.byId(var1.readVarInt());
        // TODO: UNKNOWN: this.source = (SoundSource)var1.readEnum(SoundSource.class);
        let id = buf.get_mc_var_int()?;
        let volume = buf.get_mc_f32()?;
        let pitch = buf.get_mc_f32()?;
        return Ok(SoundEntityPacket { sound, source, id, volume, pitch });
    }
}
