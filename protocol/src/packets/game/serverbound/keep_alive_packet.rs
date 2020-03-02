use crate::network::*;
use crate::packet::*;
use basin2_lib::result::*;
use bytes::BytesMut;

#[derive(PartialEq, Clone, Debug)]
pub struct KeepAlivePacket {
    pub id: i64,
}

impl CodablePacket for KeepAlivePacket {
    fn encode(self, buf: &mut BytesMut) {
        buf.set_mc_i64(self.id);
    }

    fn decode(buf: &mut BytesMut) -> Result<Self>
    where
        Self: Sized,
    {
        let id = buf.get_mc_i64()?;
        return Ok(KeepAlivePacket { id });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::packet::test::*;

    #[test]
    fn test_cycle() -> Result<()> {
        cycle(KeepAlivePacket { id: 4564321 })
    }
}
