use crate::network::*;
use crate::packet::*;
use basin2_lib::result::*;
use bytes::BytesMut;

#[derive(PartialEq, Clone, Debug)]
pub struct SetEntityLinkPacket {
    pub sourceId: i32,
    pub destId: i32,
}

impl CodablePacket for SetEntityLinkPacket {
    fn encode(self, buf: &mut BytesMut) {
        buf.set_mc_i32(self.sourceId);
        buf.set_mc_i32(self.destId);
    }

    fn decode(buf: &mut BytesMut) -> Result<Self>
    where
        Self: Sized,
    {
        let sourceId = buf.get_mc_i32()?;
        let destId = buf.get_mc_i32()?;
        return Ok(SetEntityLinkPacket { sourceId, destId });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::packet::test::*;

    #[test]
    fn test_cycle() -> Result<()> {
        cycle(SetEntityLinkPacket {
            sourceId: 3456,
            destId: 7890,
        })
    }
}
