
use crate::packet::*;
use crate::network::*;
use bytes::BytesMut;
use uuid::Uuid;
use crate::result::*;

pub struct LockDifficultyPacket {
    pub locked: bool,
}

impl CodablePacket for LockDifficultyPacket {
    fn encode(self, buf: &mut BytesMut) {
        buf.set_mc_bool(self.locked);
    }

    fn decode(buf: &mut BytesMut) -> Result<Self> where Self: Sized {
        let locked = buf.get_mc_bool()?;
        return Ok(LockDifficultyPacket { locked });
    }
}
