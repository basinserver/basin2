
use crate::packet::*;
use crate::network::*;
use bytes::BytesMut;
use uuid::Uuid;
use crate::result::*;

pub struct LoginPacket {
    pub playerId: i32,
    pub seed: i64,
    pub hardcore: bool,
    pub gameType: GameType,
    pub dimension: String,
    pub maxPlayers: i32,
    pub levelType: String,
    pub chunkRadius: i32,
    pub reducedDebugInfo: bool,
    pub showDeathScreen: bool,
}

impl CodablePacket for LoginPacket {
    fn encode(self, buf: &mut BytesMut) {
        /* TODO: NOT FOUND */
    }

    fn decode(buf: &mut BytesMut) -> Result<Self> where Self: Sized {
        /* TODO: NOT FOUND */
        return Ok(LoginPacket { playerId, seed, hardcore, gameType, dimension, maxPlayers, levelType, chunkRadius, reducedDebugInfo, showDeathScreen });
    }
}
