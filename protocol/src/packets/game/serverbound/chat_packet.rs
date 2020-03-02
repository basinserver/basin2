use crate::network::*;
use crate::packet::*;
use basin2_lib::result::*;
use bytes::BytesMut;

#[derive(PartialEq, Clone, Debug)]
pub struct ChatPacket {
    pub message: String,
}

impl CodablePacket for ChatPacket {
    fn encode(self, buf: &mut BytesMut) {
        buf.set_mc_string(self.message);
    }

    fn decode(buf: &mut BytesMut) -> Result<Self>
    where
        Self: Sized,
    {
        let message = buf.get_mc_string(256)?;
        return Ok(ChatPacket { message });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::packet::test::*;

    #[test]
    fn test_cycle() -> Result<()> {
        cycle(ChatPacket {
            message: "test".to_string(),
        })
    }
}
