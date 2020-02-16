use crate::network::*;
use crate::packet::*;
use crate::result::*;
use bytes::BytesMut;

#[derive(PartialEq, Clone, Debug)]
pub struct SetChunkCacheRadiusPacket {
    pub radius: i32,
}

impl CodablePacket for SetChunkCacheRadiusPacket {
    fn encode(self, buf: &mut BytesMut) {
        buf.set_mc_var_int(self.radius);
    }

    fn decode(buf: &mut BytesMut) -> Result<Self>
    where
        Self: Sized,
    {
        let radius = buf.get_mc_var_int()?;
        return Ok(SetChunkCacheRadiusPacket { radius });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::packet::test::*;

    #[test]
    fn test_cycle() -> Result<()> {
        cycle(SetChunkCacheRadiusPacket { radius: 12 })
    }
}
