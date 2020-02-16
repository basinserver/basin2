use crate::network::*;
use crate::packet::*;
use crate::result::*;
use bytes::BytesMut;

#[derive(PartialEq, Clone, Debug)]
pub struct ChangeDifficultyPacket {
    pub difficulty: Difficulty,
}

impl CodablePacket for ChangeDifficultyPacket {
    fn encode(self, buf: &mut BytesMut) {
        buf.set_mc_u8(self.difficulty as u8);
    }

    fn decode(buf: &mut BytesMut) -> Result<Self>
    where
        Self: Sized,
    {
        let difficulty: Difficulty = buf.get_mc_enum_u8()?;
        return Ok(ChangeDifficultyPacket { difficulty });
    }
}
