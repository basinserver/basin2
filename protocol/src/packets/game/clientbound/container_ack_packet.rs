use crate::network::*;
use crate::packet::*;
use basin2_lib::result::*;
use bytes::BytesMut;

#[derive(PartialEq, Clone, Debug)]
pub struct ContainerAckPacket {
    pub containerId: u8,
    pub uid: i16,
    pub accepted: bool,
}

impl CodablePacket for ContainerAckPacket {
    fn encode(self, buf: &mut BytesMut) {
        buf.set_mc_u8(self.containerId);
        buf.set_mc_i16(self.uid);
        buf.set_mc_bool(self.accepted);
    }

    fn decode(buf: &mut BytesMut) -> Result<Self>
    where
        Self: Sized,
    {
        let containerId = buf.get_mc_u8()?;
        let uid = buf.get_mc_i16()?;
        let accepted = buf.get_mc_bool()?;
        return Ok(ContainerAckPacket {
            containerId,
            uid,
            accepted,
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::packet::test::*;

    #[test]
    fn test_cycle() -> Result<()> {
        cycle(ContainerAckPacket {
            containerId: 12,
            uid: 5422,
            accepted: true,
        })
    }
}
