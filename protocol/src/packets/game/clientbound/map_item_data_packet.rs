
use crate::packet::*;
use crate::network::*;
use bytes::BytesMut;
use uuid::Uuid;
use crate::result::*;

pub struct MapItemDataPacket {
    pub mapId: i32,
    pub scale: u8,
    pub trackingPosition: bool,
    pub locked: bool,
    pub decorations: Vec<MapDecoration>,
    pub startX: i32,
    pub startY: i32,
    pub width: i32,
    pub height: i32,
    pub mapColors: Vec<u8>,
}

impl CodablePacket for MapItemDataPacket {
    fn encode(self, buf: &mut BytesMut) {
        /* TODO: NOT FOUND */
    }

    fn decode(buf: &mut BytesMut) -> Result<Self> where Self: Sized {
        /* TODO: NOT FOUND */
        return Ok(MapItemDataPacket { mapId, scale, trackingPosition, locked, decorations, startX, startY, width, height, mapColors });
    }
}
