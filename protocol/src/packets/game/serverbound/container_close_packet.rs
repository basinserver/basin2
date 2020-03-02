use crate::network::*;
use crate::packet::*;
use basin2_lib::result::*;
use bytes::BytesMut;

#[derive(PartialEq, Clone, Debug)]
pub struct ContainerClosePacket {
    pub containerId: u8,
}

impl CodablePacket for ContainerClosePacket {
    fn encode(self, buf: &mut BytesMut) {
        buf.set_mc_u8(self.containerId);
    }

    fn decode(buf: &mut BytesMut) -> Result<Self>
    where
        Self: Sized,
    {
        let containerId = buf.get_mc_u8()?;
        return Ok(ContainerClosePacket { containerId });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::packet::test::*;

    #[test]
    fn test_cycle() -> Result<()> {
        cycle(ContainerClosePacket { containerId: 123 })
    }
}
