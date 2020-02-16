use crate::network::*;
use crate::packet::*;
use crate::result::*;
use bytes::BytesMut;

#[derive(PartialEq, Clone, Debug)]
pub struct PingRequestPacket {
    pub time: i64,
}

impl CodablePacket for PingRequestPacket {
    fn encode(self, buf: &mut BytesMut) {
        buf.set_mc_i64(self.time);
    }

    fn decode(buf: &mut BytesMut) -> Result<Self>
    where
        Self: Sized,
    {
        let time = buf.get_mc_i64()?;
        return Ok(PingRequestPacket { time });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::packet::test::*;

    #[test]
    fn test_cycle() -> Result<()> {
        cycle(PingRequestPacket {
            time: 1234567,
        })
    }
}