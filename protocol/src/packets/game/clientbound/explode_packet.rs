
use crate::packet::*;
use crate::network::*;
use bytes::BytesMut;
use uuid::Uuid;
use crate::result::*;

pub struct ExplodePacket {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub power: f32,
    pub toBlow: undefined,
    pub knockbackX: f32,
    pub knockbackY: f32,
    pub knockbackZ: f32,
}

impl CodablePacket for ExplodePacket {
    fn encode(self, buf: &mut BytesMut) {
        /* TODO: NOT FOUND */
    }

    fn decode(buf: &mut BytesMut) -> Result<Self> where Self: Sized {
        /* TODO: NOT FOUND */
        return Ok(ExplodePacket { x, y, z, power, toBlow, knockbackX, knockbackY, knockbackZ });
    }
}
