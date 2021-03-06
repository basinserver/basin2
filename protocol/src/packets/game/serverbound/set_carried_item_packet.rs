use crate::network::*;
use crate::packet::*;
use basin2_lib::result::*;
use bytes::BytesMut;

#[derive(PartialEq, Clone, Debug)]
pub struct SetCarriedItemPacket {
    pub slot: i16,
}

impl CodablePacket for SetCarriedItemPacket {
    fn encode(self, buf: &mut BytesMut) {
        buf.set_mc_i16(self.slot);
    }

    fn decode(buf: &mut BytesMut) -> Result<Self>
    where
        Self: Sized,
    {
        let slot = buf.get_mc_i16()?;
        return Ok(SetCarriedItemPacket { slot });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::packet::test::*;

    #[test]
    fn test_cycle() -> Result<()> {
        cycle(SetCarriedItemPacket { slot: 7 })
    }
}
