use crate::network::*;
use crate::packet::*;
use crate::result::*;
use bytes::BytesMut;

#[derive(PartialEq, Clone, Debug)]
pub struct MovePlayerPacket {
    pub onGround: bool,
}

impl CodablePacket for MovePlayerPacket {
    fn encode(self, buf: &mut BytesMut) {
        buf.set_mc_bool(self.onGround);
    }

    fn decode(buf: &mut BytesMut) -> Result<Self>
    where
        Self: Sized,
    {
        let onGround = buf.get_mc_bool()?;
        return Ok(MovePlayerPacket { onGround });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::packet::test::*;

    #[test]
    fn test_cycle() -> Result<()> {
        cycle(MovePlayerPacket { onGround: false })
    }
}
