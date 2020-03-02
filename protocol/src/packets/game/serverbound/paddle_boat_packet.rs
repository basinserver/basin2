use crate::network::*;
use crate::packet::*;
use basin2_lib::result::*;
use bytes::BytesMut;

#[derive(PartialEq, Clone, Debug)]
pub struct PaddleBoatPacket {
    pub left: bool,
    pub right: bool,
}

impl CodablePacket for PaddleBoatPacket {
    fn encode(self, buf: &mut BytesMut) {
        buf.set_mc_bool(self.left);
        buf.set_mc_bool(self.right);
    }

    fn decode(buf: &mut BytesMut) -> Result<Self>
    where
        Self: Sized,
    {
        let left = buf.get_mc_bool()?;
        let right = buf.get_mc_bool()?;
        return Ok(PaddleBoatPacket { left, right });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::packet::test::*;

    #[test]
    fn test_cycle() -> Result<()> {
        cycle(PaddleBoatPacket {
            left: true,
            right: false,
        })
    }
}
