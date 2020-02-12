
use crate::packet::*;
use crate::network::*;
use bytes::BytesMut;
use uuid::Uuid;
use crate::result::*;

pub struct LevelParticlesPacket {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub xDist: f32,
    pub yDist: f32,
    pub zDist: f32,
    pub maxSpeed: f32,
    pub count: i32,
    pub overrideLimiter: bool,
    pub particle: undefined,
}

impl CodablePacket for LevelParticlesPacket {
    fn encode(self, buf: &mut BytesMut) {
        // TODO: UNKNOWN: var1.writeInt(Registry.PARTICLE_TYPE.getId(this.particle.getType()));
        buf.set_mc_bool(self.overrideLimiter);
        buf.set_mc_f64(self.x);
        buf.set_mc_f64(self.y);
        buf.set_mc_f64(self.z);
        buf.set_mc_f32(self.xDist);
        buf.set_mc_f32(self.yDist);
        buf.set_mc_f32(self.zDist);
        buf.set_mc_f32(self.maxSpeed);
        buf.set_mc_i32(self.count);
        // TODO: EXTRA: this.particle.writeToNetwork(var1);
    }

    fn decode(buf: &mut BytesMut) -> Result<Self> where Self: Sized {
        /* TODO: NOT FOUND */
        return Ok(LevelParticlesPacket { x, y, z, xDist, yDist, zDist, maxSpeed, count, overrideLimiter, particle });
    }
}
