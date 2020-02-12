
use crate::packet::*;
use crate::network::*;
use bytes::BytesMut;
use uuid::Uuid;
use crate::result::*;

pub struct RespawnPacket {
    pub dimension: String,
    pub seed: i64,
    pub playerGameType: GameType,
    pub levelType: String,
}

impl CodablePacket for RespawnPacket {
    fn encode(self, buf: &mut BytesMut) {
        // TODO: UNKNOWN: var1.writeInt(this.dimension.getId());
        buf.set_mc_i64(self.seed);
        // TODO: UNKNOWN: var1.writeByte(this.playerGameType.getId());
        // TODO: UNKNOWN: var1.writeUtf(this.levelType.getName());
    }

    fn decode(buf: &mut BytesMut) -> Result<Self> where Self: Sized {
        /* TODO: NOT FOUND */
        return Ok(RespawnPacket { dimension, seed, playerGameType, levelType });
    }
}
