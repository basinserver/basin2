
use crate::packet::*;
use crate::network::*;
use bytes::BytesMut;
use uuid::Uuid;
use crate::result::*;

pub struct LevelChunkPacket {
    pub x: i32,
    pub z: i32,
    pub availableSections: i32,
    pub heightmaps: Nbt,
    pub biomes: Vec<i32>,
    pub buffer: Vec<u8>,
    pub blockEntitiesTags: undefined,
    pub fullChunk: bool,
}

impl CodablePacket for LevelChunkPacket {
    fn encode(self, buf: &mut BytesMut) {
        /* TODO: NOT FOUND */
    }

    fn decode(buf: &mut BytesMut) -> Result<Self> where Self: Sized {
        /* TODO: NOT FOUND */
        return Ok(LevelChunkPacket { x, z, availableSections, heightmaps, biomes, buffer, blockEntitiesTags, fullChunk });
    }
}
