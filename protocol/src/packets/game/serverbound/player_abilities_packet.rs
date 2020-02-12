
use crate::packet::*;
use crate::network::*;
use bytes::BytesMut;
use uuid::Uuid;
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
        /* TODO: NOT FOUND */
    }

    fn decode(buf: &mut BytesMut) -> Result<Self> where Self: Sized {
        // TODO: UNKNOWN: byte var2 = var1.readByte();
        // TODO: EXTRA: this.setInvulnerable((var2 & 1) > 0);
        // TODO: EXTRA: this.setFlying((var2 & 2) > 0);
        // TODO: EXTRA: this.setCanFly((var2 & 4) > 0);
        // TODO: EXTRA: this.setInstabuild((var2 & 8) > 0);
        // TODO: UNKNOWN: this.setFlyingSpeed(var1.readFloat());
        // TODO: UNKNOWN: this.setWalkingSpeed(var1.readFloat());
        return Ok(PlayerAbilitiesPacket { invulnerable, isFlying, canFly, instabuild, flyingSpeed, walkingSpeed });
    }
}
