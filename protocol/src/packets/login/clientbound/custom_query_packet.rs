use crate::network::*;
use crate::packet::*;
use crate::result::*;
use bytes::BytesMut;

pub struct CustomQueryPacket {
    pub transactionId: i32,
    pub identifier: ResourceLocation,
    pub data: BytesMut,
}

impl CodablePacket for CustomQueryPacket {
    fn encode(self, buf: &mut BytesMut) {
        buf.set_mc_var_int(self.transactionId);
        buf.set_mc_string(self.identifier);
        buf.unsplit(self.data);
    }

    fn decode(buf: &mut BytesMut) -> Result<Self>
    where
        Self: Sized,
    {
        let transactionId = buf.get_mc_var_int()?;
        let identifier = buf.get_mc_string(32767)?;
        if buf.len() > 1048576 {
            return Err(Box::new(IoError::from(ErrorKind::InvalidData)));
        }
        let data = buf.clone_bounded(1048576)?;
        return Ok(CustomQueryPacket {
            transactionId,
            identifier,
            data,
        });
    }
}
