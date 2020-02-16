use crate::network::*;
use crate::packet::*;
use crate::result::*;
use bytes::BytesMut;

#[derive(PartialEq, Clone, Debug)]
pub struct ContainerSetDataPacket {
    pub containerId: u8,
    pub id: i16,
    pub value: i16,
}

impl CodablePacket for ContainerSetDataPacket {
    fn encode(self, buf: &mut BytesMut) {
        buf.set_mc_u8(self.containerId);
        buf.set_mc_i16(self.id);
        buf.set_mc_i16(self.value);
    }

    fn decode(buf: &mut BytesMut) -> Result<Self>
    where
        Self: Sized,
    {
        let containerId = buf.get_mc_u8()?;
        let id = buf.get_mc_i16()?;
        let value = buf.get_mc_i16()?;
        return Ok(ContainerSetDataPacket {
            containerId,
            id,
            value,
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::packet::test::*;

    #[test]
    fn test_cycle() -> Result<()> {
        cycle(ContainerSetDataPacket {
            containerId: 12,
            id: 4352,
            value: 7852,
        })
    }
}