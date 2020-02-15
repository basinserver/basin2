
use crate::packet::*;
use crate::network::*;
use bytes::BytesMut;
use crate::result::*;

pub struct PlayerAbilitiesPacket {
    pub invulnerable: bool,
    pub isFlying: bool,
    pub canFly: bool,
    pub instabuild: bool,
    pub flyingSpeed: f32,
    pub walkingSpeed: f32,
}

impl CodablePacket for PlayerAbilitiesPacket {
    fn encode(self, buf: &mut BytesMut) {
        let mut flags = 0;
        if self.invulnerable {
            flags |= 1;
        }
        if self.isFlying {
            flags |= 2;
        }
        if self.canFly {
            flags |= 4;
        }
        if self.instabuild {
            flags |= 8;
        }
        buf.set_mc_u8(flags);
        buf.set_mc_f32(self.flyingSpeed);
        buf.set_mc_f32(self.walkingSpeed);
    }

    fn decode(buf: &mut BytesMut) -> Result<Self> where Self: Sized {
        let flags = buf.get_mc_u8()?;
        let invulnerable = (flags & 1) > 0;
        let isFlying = (flags & 2) > 0;
        let canFly = (flags & 4) > 0;
        let instabuild = (flags & 8) > 0;
        let flyingSpeed = buf.get_mc_f32()?;
        let walkingSpeed = buf.get_mc_f32()?;
        return Ok(PlayerAbilitiesPacket { invulnerable, isFlying, canFly, instabuild, flyingSpeed, walkingSpeed });
    }
}
