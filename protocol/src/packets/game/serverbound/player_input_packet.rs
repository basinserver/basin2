use crate::network::*;
use crate::packet::*;
use basin2_lib::result::*;
use bytes::BytesMut;

#[derive(PartialEq, Clone, Debug)]
pub struct PlayerInputPacket {
    pub xxa: f32,
    pub zza: f32,
    pub isJumping: bool,
    pub isShiftKeyDown: bool,
}

impl CodablePacket for PlayerInputPacket {
    fn encode(self, buf: &mut BytesMut) {
        buf.set_mc_f32(self.xxa);
        buf.set_mc_f32(self.zza);
        let mut flags = 0;
        if self.isJumping {
            flags |= 1;
        }
        if self.isShiftKeyDown {
            flags |= 2;
        }
        buf.set_mc_u8(flags);
    }

    fn decode(buf: &mut BytesMut) -> Result<Self>
    where
        Self: Sized,
    {
        let xxa = buf.get_mc_f32()?;
        let zza = buf.get_mc_f32()?;
        let flags = buf.get_mc_u8()?;
        let isJumping = (flags & 1) > 0;
        let isShiftKeyDown = (flags & 2) > 0;
        return Ok(PlayerInputPacket {
            xxa,
            zza,
            isJumping,
            isShiftKeyDown,
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::packet::test::*;

    #[test]
    fn test_cycle() -> Result<()> {
        cycle(PlayerInputPacket {
            xxa: 12345.0,
            zza: 54321.1253245,
            isJumping: false,
            isShiftKeyDown: true,
        })
    }
}
