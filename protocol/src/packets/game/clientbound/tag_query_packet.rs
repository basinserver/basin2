use basin2_lib::Nbt;
use crate::network::*;
use crate::packet::*;
use basin2_lib::result::*;
use bytes::BytesMut;

#[derive(PartialEq, Clone, Debug)]
pub struct TagQueryPacket {
    pub transactionId: i32,
    pub tag: Nbt,
}

impl CodablePacket for TagQueryPacket {
    fn encode(self, buf: &mut BytesMut) {
        buf.set_mc_var_int(self.transactionId);
        buf.set_mc_nbt(self.tag);
    }

    fn decode(buf: &mut BytesMut) -> Result<Self>
    where
        Self: Sized,
    {
        let transactionId = buf.get_mc_var_int()?;
        let tag = buf.get_mc_nbt()?;
        return Ok(TagQueryPacket { transactionId, tag });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::packet::test::*;

    #[test]
    fn test_cycle() -> Result<()> {
        cycle(TagQueryPacket {
            transactionId: 234456,
            tag: Nbt::make_singleton_compound("test nbt".to_string(), Nbt::Float(47.0)),
        })
    }
}
