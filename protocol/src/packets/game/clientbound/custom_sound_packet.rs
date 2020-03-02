use crate::network::*;
use crate::packet::*;
use basin2_lib::result::*;
use bytes::BytesMut;

#[derive(PartialEq, Clone, Debug)]
pub struct CustomSoundPacket {
    pub name: ResourceLocation,
    pub source: SoundSource,
    pub x: i32,
    pub y: i32,
    pub z: i32,
    pub volume: f32,
    pub pitch: f32,
}

impl CodablePacket for CustomSoundPacket {
    fn encode(self, buf: &mut BytesMut) {
        buf.set_mc_string(self.name);
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
        let name = buf.get_mc_string(32767)?;
        let source: SoundSource = buf.get_mc_enum()?;
        let x = buf.get_mc_i32()?;
        let y = buf.get_mc_i32()?;
        let z = buf.get_mc_i32()?;
        let volume = buf.get_mc_f32()?;
        let pitch = buf.get_mc_f32()?;
        return Ok(CustomSoundPacket {
            name,
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
        cycle(CustomSoundPacket {
            name: "test ident".to_string(),
            source: SoundSource::Weather,
            x: 123,
            y: 128,
            z: -1200,
            volume: 1.0,
            pitch: 1.0,
        })
    }
}
