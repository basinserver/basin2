
use crate::packet::*;
use crate::network::*;
use bytes::BytesMut;
use uuid::Uuid;
use crate::result::*;

pub struct LightUpdatePacket {
    pub x: i32,
    pub z: i32,
    pub skyYMask: i32,
    pub blockYMask: i32,
    pub emptySkyYMask: i32,
    pub emptyBlockYMask: i32,
    pub skyUpdates: undefined,
    pub blockUpdates: undefined,
}

impl CodablePacket for LightUpdatePacket {
    fn encode(self, buf: &mut BytesMut) {
        /* TODO: NOT FOUND */
    }

    fn decode(buf: &mut BytesMut) -> Result<Self> where Self: Sized {
        /* TODO: NOT FOUND */
        return Ok(LightUpdatePacket { x, z, skyYMask, blockYMask, emptySkyYMask, emptyBlockYMask, skyUpdates, blockUpdates });
    }
}
