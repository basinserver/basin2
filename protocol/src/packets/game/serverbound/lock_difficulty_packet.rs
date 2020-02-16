use crate::network::*;
use crate::packet::*;
use crate::result::*;
use bytes::BytesMut;

#[derive(PartialEq, Clone, Debug)]
pub struct LockDifficultyPacket {
    pub locked: bool,
}

impl CodablePacket for LockDifficultyPacket {
    fn encode(self, buf: &mut BytesMut) {
        buf.set_mc_bool(self.locked);
    }

    fn decode(buf: &mut BytesMut) -> Result<Self>
    where
        Self: Sized,
    {
        let locked = buf.get_mc_bool()?;
        return Ok(LockDifficultyPacket { locked });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::packet::test::*;

    #[test]
    fn test_cycle() -> Result<()> {
        cycle(LockDifficultyPacket {
            locked: false,
        })
    }
}