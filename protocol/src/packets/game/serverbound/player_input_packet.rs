
use crate::packet::*;
use crate::network::*;
use bytes::BytesMut;
use uuid::Uuid;
use crate::result::*;

pub struct PlayerInputPacket {
    pub xxa: f32,
    pub zza: f32,
    pub isJumping: bool,
    pub isShiftKeyDown: bool,
}

impl CodablePacket for PlayerInputPacket {
    fn encode(self, buf: &mut BytesMut) {
        /* TODO: NOT FOUND */
    }

    fn decode(buf: &mut BytesMut) -> Result<Self> where Self: Sized {
        let xxa = buf.get_mc_f32()?;
        let zza = buf.get_mc_f32()?;
        // TODO: UNKNOWN: byte var2 = var1.readByte();
        // TODO: EXTRA: this.isJumping = (var2 & 1) > 0;
        // TODO: EXTRA: this.isShiftKeyDown = (var2 & 2) > 0;
        return Ok(PlayerInputPacket { xxa, zza, isJumping, isShiftKeyDown });
    }
}
