
use crate::packet::*;
use crate::network::*;
use bytes::BytesMut;
use uuid::Uuid;
use crate::result::*;

pub struct ChangeDifficultyPacket {
    pub difficulty: Difficulty,
}

impl CodablePacket for ChangeDifficultyPacket {
    fn encode(self, buf: &mut BytesMut) {
        // TODO: UNKNOWN: var1.writeByte(this.difficulty.getId());
    }

    fn decode(buf: &mut BytesMut) -> Result<Self> where Self: Sized {
        // TODO: UNKNOWN: this.difficulty = Difficulty.byId(var1.readUnsignedByte());
        return Ok(ChangeDifficultyPacket { difficulty });
    }
}
