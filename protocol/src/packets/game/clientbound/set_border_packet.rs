
use crate::packet::*;
use crate::network::*;
use bytes::BytesMut;
use uuid::Uuid;
use crate::result::*;

pub struct SetBorderPacket {
    pub type: SetBorderPacketType,
    pub newAbsoluteMaxSize: i32,
    pub newCenterX: f64,
    pub newCenterZ: f64,
    pub newSize: f64,
    pub oldSize: f64,
    pub lerpTime: i64,
    pub warningTime: i32,
    pub warningBlocks: i32,
}

impl CodablePacket for SetBorderPacket {
    fn encode(self, buf: &mut BytesMut) {
        /* TODO: NOT FOUND */
    }

    fn decode(buf: &mut BytesMut) -> Result<Self> where Self: Sized {
        /* TODO: NOT FOUND */
        return Ok(SetBorderPacket { type, newAbsoluteMaxSize, newCenterX, newCenterZ, newSize, oldSize, lerpTime, warningTime, warningBlocks });
    }
}
