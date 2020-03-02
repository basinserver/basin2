use crate::network::*;
use crate::packet::*;
use basin2_lib::result::*;
use bytes::BytesMut;

#[derive(PartialEq, Clone, Debug)]
pub struct ResourcePackPacket {
    pub url: String,
    pub hash: String,
}

impl CodablePacket for ResourcePackPacket {
    fn encode(self, buf: &mut BytesMut) {
        buf.set_mc_string(self.url);
        buf.set_mc_string(self.hash);
    }

    fn decode(buf: &mut BytesMut) -> Result<Self>
    where
        Self: Sized,
    {
        let url = buf.get_mc_string(32767)?;
        let hash = buf.get_mc_string(40)?;
        return Ok(ResourcePackPacket { url, hash });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::packet::test::*;

    #[test]
    fn test_cycle() -> Result<()> {
        cycle(ResourcePackPacket {
            url: "some url".to_string(),
            hash: "456464gbfsdgh56".to_string(),
        })
    }
}
