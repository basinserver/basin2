
use crate::packet::*;
use crate::network::*;
use bytes::BytesMut;
use uuid::Uuid;
use crate::result::*;

pub struct ChangeDifficultyPacket {
    pub difficulty: Difficulty,
    pub locked: bool,
}

impl CodablePacket for ChangeDifficultyPacket {
    fn encode(self, buf: &mut BytesMut) {
        // TODO: UNKNOWN: var1.writeByte(this.difficulty.getId());
        buf.set_mc_bool(self.locked);
    }

    fn decode(buf: &mut BytesMut) -> Result<Self> where Self: Sized {
        // TODO: UNKNOWN: this.difficulty = Difficulty.byId(var1.readUnsignedByte());
        let locked = buf.get_mc_bool()?;
        return Ok(ChangeDifficultyPacket { difficulty, locked });
    }
}
