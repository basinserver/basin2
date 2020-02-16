use crate::network::*;
use crate::packet::*;
use crate::result::*;
use bytes::BytesMut;

#[derive(PartialEq, Clone, Debug)]
pub struct LoginCompressionPacket {
    pub compressionThreshold: i32,
}

impl CodablePacket for LoginCompressionPacket {
    fn encode(self, buf: &mut BytesMut) {
        buf.set_mc_var_int(self.compressionThreshold);
    }

    fn decode(buf: &mut BytesMut) -> Result<Self>
    where
        Self: Sized,
    {
        let compressionThreshold = buf.get_mc_var_int()?;
        return Ok(LoginCompressionPacket {
            compressionThreshold,
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::packet::test::*;

    #[test]
    fn test_cycle() -> Result<()> {
        cycle(LoginCompressionPacket {
            compressionThreshold: 256,
        })
    }
}
