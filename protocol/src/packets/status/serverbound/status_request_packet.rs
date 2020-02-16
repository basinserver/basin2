use crate::packet::*;
use crate::result::*;
use bytes::BytesMut;

#[derive(PartialEq, Clone, Debug)]
pub struct StatusRequestPacket {}

impl CodablePacket for StatusRequestPacket {
    fn encode(self, _buf: &mut BytesMut) {}

    fn decode(_buf: &mut BytesMut) -> Result<Self>
    where
        Self: Sized,
    {
        return Ok(StatusRequestPacket {});
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::packet::test::*;

    #[test]
    fn test_cycle() -> Result<()> {
        cycle(StatusRequestPacket {})
    }
}
