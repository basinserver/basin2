use crate::network::*;
use crate::packet::*;
use crate::result::*;
use bytes::BytesMut;

#[derive(PartialEq, Clone, Debug)]
pub struct SoundEntityPacket {
    pub sound: SoundEvent,
    pub source: SoundSource,
    pub id: i32,
    pub volume: f32,
    pub pitch: f32,
}

impl CodablePacket for SoundEntityPacket {
    fn encode(self, buf: &mut BytesMut) {
        buf.set_mc_var_int(self.sound);
        buf.set_mc_var_int(self.source as i32);
        buf.set_mc_var_int(self.id);
        buf.set_mc_f32(self.volume);
        buf.set_mc_f32(self.pitch);
    }

    fn decode(buf: &mut BytesMut) -> Result<Self>
    where
        Self: Sized,
    {
        let sound = buf.get_mc_var_int()?;
        let source: SoundSource = buf.get_mc_enum()?;
        let id = buf.get_mc_var_int()?;
        let volume = buf.get_mc_f32()?;
        let pitch = buf.get_mc_f32()?;
        return Ok(SoundEntityPacket {
            sound,
            source,
            id,
            volume,
            pitch,
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::packet::test::*;

    #[test]
    fn test_cycle() -> Result<()> {
        cycle(SoundEntityPacket {
            sound: 345,
            source: SoundSource::Weather,
            id: 546334,
            volume: 1.5,
            pitch: 1.0,
        })
    }
}
