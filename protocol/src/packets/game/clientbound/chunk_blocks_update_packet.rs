
use crate::packet::*;
use crate::network::*;
use bytes::BytesMut;
use uuid::Uuid;
use crate::result::*;

pub struct ChunkBlocksUpdatePacket {
    pub chunkPos: undefined,
    pub updates: Vec<ChunkBlocksUpdatePacketBlockUpdate>,
}

impl CodablePacket for ChunkBlocksUpdatePacket {
    fn encode(self, buf: &mut BytesMut) {
        /* TODO: NOT FOUND */
    }

    fn decode(buf: &mut BytesMut) -> Result<Self> where Self: Sized {
        /* TODO: NOT FOUND */
        return Ok(ChunkBlocksUpdatePacket { chunkPos, updates });
    }
}
