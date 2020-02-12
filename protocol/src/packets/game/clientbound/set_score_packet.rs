
use crate::packet::*;
use crate::network::*;
use bytes::BytesMut;
use uuid::Uuid;
use crate::result::*;

pub struct SetScorePacket {
    pub owner: String,
    pub objectiveName: String,
    pub score: i32,
    pub method: ServerScoreboardMethod,
}

impl CodablePacket for SetScorePacket {
    fn encode(self, buf: &mut BytesMut) {
        /* TODO: NOT FOUND */
    }

    fn decode(buf: &mut BytesMut) -> Result<Self> where Self: Sized {
        /* TODO: NOT FOUND */
        return Ok(SetScorePacket { owner, objectiveName, score, method });
    }
}
