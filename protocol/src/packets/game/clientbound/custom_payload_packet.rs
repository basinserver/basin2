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
        let identifier = buf.get_mc_string(1048576)?;
        let data = buf.clone();
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
            identifier: "test ident".to_string(),
            data: BytesMut::from(&vec![0x50, 0x60][..]),
        })
    }
}