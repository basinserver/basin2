
use crate::packet::*;
use crate::network::*;
use bytes::BytesMut;
use crate::result::*;

pub struct StopSoundPacket {
    pub name: Option<ResourceLocation>,
    pub source: Option<SoundSource>,
}

impl CodablePacket for StopSoundPacket {
    fn encode(self, buf: &mut BytesMut) {
        let mut flags = 0;
        if self.source.is_some() {
            flags |= 1;
        }
        if self.name.is_some() {
            flags |= 1;
        }
        buf.set_mc_u8(flags);
        if self.source.is_some() {
            buf.set_mc_var_int(self.source.unwrap() as i32);
        }
        if self.name.is_some() {
            buf.set_mc_string(self.name.unwrap());
        }
    }

    fn decode(buf: &mut BytesMut) -> Result<Self> where Self: Sized {
        let flags = buf.get_mc_u8()?;
        let source = match flags {
            x if (x & 1) > 0 => {
                Some(buf.get_mc_enum::<SoundSource>()?)
            }
            _ => None,
        };
        let name = match flags {
            x if (x & 2) > 0 => {
                Some(buf.get_mc_string(32767)?)
            }
            _ => None,
        };
        return Ok(StopSoundPacket { name, source });
    }
}
