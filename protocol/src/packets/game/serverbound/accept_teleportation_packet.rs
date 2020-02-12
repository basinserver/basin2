
use crate::packet::*;
use crate::network::*;
use bytes::BytesMut;
use uuid::Uuid;
use crate::result::*;

pub struct AcceptTeleportationPacket {
    pub id: i32,
}

impl CodablePacket for AcceptTeleportationPacket {
    fn encode(self, buf: &mut BytesMut) {
        buf.set_mc_var_int(self.id);
    }

    fn decode(buf: &mut BytesMut) -> Result<Self> where Self: Sized {
        let id = buf.get_mc_var_int()?;
        return Ok(AcceptTeleportationPacket { id });
    }
}
