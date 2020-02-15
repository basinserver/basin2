
use crate::packet::*;
use crate::network::*;
use bytes::BytesMut;
use crate::result::*;

pub struct PaddleBoatPacket {
    pub left: bool,
    pub right: bool,
}

impl CodablePacket for PaddleBoatPacket {
    fn encode(self, buf: &mut BytesMut) {
        buf.set_mc_bool(self.left);
        buf.set_mc_bool(self.right);
    }

    fn decode(buf: &mut BytesMut) -> Result<Self> where Self: Sized {
        let left = buf.get_mc_bool()?;
        let right = buf.get_mc_bool()?;
        return Ok(PaddleBoatPacket { left, right });
    }
}
