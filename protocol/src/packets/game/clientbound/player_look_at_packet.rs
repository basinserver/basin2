
use crate::packet::*;
use crate::network::*;
use bytes::BytesMut;
use uuid::Uuid;
use crate::result::*;

pub struct PlayerLookAtPacket {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub entity: i32,
    pub fromAnchor: EntityAnchor,
    pub toAnchor: EntityAnchor,
    pub atEntity: bool,
}

impl CodablePacket for PlayerLookAtPacket {
    fn encode(self, buf: &mut BytesMut) {
        /* TODO: NOT FOUND */
    }

    fn decode(buf: &mut BytesMut) -> Result<Self> where Self: Sized {
        /* TODO: NOT FOUND */
        return Ok(PlayerLookAtPacket { x, y, z, entity, fromAnchor, toAnchor, atEntity });
    }
}
