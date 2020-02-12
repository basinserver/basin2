
use crate::packet::*;
use crate::network::*;
use bytes::BytesMut;
use uuid::Uuid;
use crate::result::*;

pub struct MovePlayerPacket {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub yRot: f32,
    pub xRot: f32,
    pub onGround: bool,
    pub hasPos: bool,
    pub hasRot: bool,
}

impl CodablePacket for MovePlayerPacket {
    fn encode(self, buf: &mut BytesMut) {
        // TODO: UNKNOWN: var1.writeByte(this.onGround ? 1 : 0);
    }

    fn decode(buf: &mut BytesMut) -> Result<Self> where Self: Sized {
        // TODO: UNKNOWN: this.onGround = var1.readUnsignedByte() != 0;
        return Ok(MovePlayerPacket { x, y, z, yRot, xRot, onGround, hasPos, hasRot });
    }
}
