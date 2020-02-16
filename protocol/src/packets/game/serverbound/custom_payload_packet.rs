use crate::network::*;
use crate::packet::*;
use crate::result::*;
use bytes::BytesMut;

#[derive(PartialEq, Clone, Debug)]
pub struct CustomPayloadPacket {
    pub identifier: ResourceLocation,
    pub data: BytesMut,
}

impl CodablePacket for CustomPayloadPacket {
    fn encode(self, buf: &mut BytesMut) {
        buf.set_mc_string(self.identifier);
        buf.unsplit(self.data);
    }

    fn decode(buf: &mut BytesMut) -> Result<Self>
    where
        Self: Sized,
    {
        let identifier = buf.get_mc_string(32767)?;
        let data = buf.clone_bounded(32767)?;
        return Ok(CustomPayloadPacket { identifier, data });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::packet::test::*;

    #[test]
    fn test_cycle() -> Result<()> {
        cycle(CustomPayloadPacket {
            identifier: "test channel".to_string(),
            data: BytesMut::from(&vec![0x1f, 0x2f][..]),
        })
    }
}
