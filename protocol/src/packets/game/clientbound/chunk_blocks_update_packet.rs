use crate::network::*;
use crate::packet::*;
use crate::result::*;
use bytes::BytesMut;

pub struct ChunkBlocksUpdatePacket {
    pub chunkPos: ChunkPos,
    pub updates: Vec<ChunkBlocksUpdatePacketBlockUpdate>,
}

impl CodablePacket for ChunkBlocksUpdatePacket {
    fn encode(self, buf: &mut BytesMut) {
        buf.set_mc_var_int(self.chunkPos.x);
        buf.set_mc_var_int(self.chunkPos.y);
        buf.set_mc_var_int(self.updates.len() as i32);
        for update in self.updates {
            buf.set_mc_u16(update.offset);
            buf.set_mc_var_int(update.block);
        }
    }

    fn decode(buf: &mut BytesMut) -> Result<Self>
    where
        Self: Sized,
    {
        let chunkPos = ChunkPos {
            x: buf.get_mc_var_int()?,
            y: buf.get_mc_var_int()?,
        };
        let update_count = buf.get_mc_var_int()?;
        let mut updates: Vec<ChunkBlocksUpdatePacketBlockUpdate> = vec![];
        for _ in 0..update_count {
            updates.push(ChunkBlocksUpdatePacketBlockUpdate {
                offset: buf.get_mc_u16()?,
                block: buf.get_mc_var_int()?,
            });
        }
        return Ok(ChunkBlocksUpdatePacket { chunkPos, updates });
    }
}
