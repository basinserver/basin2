use crate::network::*;
use crate::packet::*;
use basin2_lib::result::*;
use bytes::BytesMut;

#[derive(PartialEq, Clone, Debug)]
pub struct SoundPacket {
    pub sound: SoundEvent,
    pub source: SoundSource,
    pub x: i32,
    pub y: i32,
    pub z: i32,
    pub volume: f32,
    pub pitch: f32,
}

impl CodablePacket for SoundPacket {
    fn encode(self, buf: &mut BytesMut) {
        buf.set_mc_var_int(self.sound);
        buf.set_mc_var_int(self.source as i32);
        buf.set_mc_i32(self.x);
        buf.set_mc_i32(self.y);
        buf.set_mc_i32(self.z);
        buf.set_mc_f32(self.volume);
        buf.set_mc_f32(self.pitch);
    }

    fn decode(buf: &mut BytesMut) -> Result<Self>
    where
        Self: Sized,
    {
        let sound = buf.get_mc_var_int()?;
        let source: SoundSource = buf.get_mc_enum()?;
        let x = buf.get_mc_i32()?;
        let y = buf.get_mc_i32()?;
        let z = buf.get_mc_i32()?;
        let volume = buf.get_mc_f32()?;
        let pitch = buf.get_mc_f32()?;
        return Ok(SoundPacket {
            sound,
            source,
            x,
            y,
            z,
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
        cycle(SoundPacket {
            sound: 345,
            source: SoundSource::Weather,
            x: 35634,
            y: 64,
            z: -43,
            volume: 1.5,
            pitch: 1.0,
        })
    }
}
