
use crate::packet::*;
use crate::network::*;
use bytes::BytesMut;
use crate::result::*;

pub struct CustomQueryPacket {
    pub transactionId: i32,
    pub data: Option<BytesMut>,
}

impl CodablePacket for CustomQueryPacket {
    fn encode(self, buf: &mut BytesMut) {
        buf.set_mc_var_int(self.transactionId);
        match self.data {
            Some(bytes) => {
                buf.set_mc_bool(true);
                buf.unsplit(bytes);
            }
            None => {
                buf.set_mc_bool(false);
            }
        }
    }

    fn decode(buf: &mut BytesMut) -> Result<Self> where Self: Sized {
        let transactionId = buf.get_mc_var_int()?;
        let data = if buf.get_mc_bool()? {
            Some(buf.clone_bounded(1048576)?)
        } else {
            None
        };
        return Ok(CustomQueryPacket { transactionId, data });
    }
}
