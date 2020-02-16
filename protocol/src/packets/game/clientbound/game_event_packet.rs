use crate::network::*;
use crate::packet::*;
use crate::result::*;
use bytes::BytesMut;

#[derive(PartialEq, Clone, Debug)]
pub struct GameEventPacket {
    pub event: u8,
    pub param: f32,
}

impl CodablePacket for GameEventPacket {
    fn encode(self, buf: &mut BytesMut) {
        buf.set_mc_u8(self.event);
        buf.set_mc_f32(self.param);
    }

    fn decode(buf: &mut BytesMut) -> Result<Self>
    where
        Self: Sized,
    {
        let event = buf.get_mc_u8()?;
        let param = buf.get_mc_f32()?;
        return Ok(GameEventPacket { event, param });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::packet::test::*;

    #[test]
    fn test_cycle() -> Result<()> {
        cycle(GameEventPacket {
            event: 12,
            param: 124563.5,
        })
    }
}