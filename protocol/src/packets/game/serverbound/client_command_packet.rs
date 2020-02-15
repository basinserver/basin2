
use crate::packet::*;
use crate::network::*;
use bytes::BytesMut;
use crate::result::*;

pub struct ClientCommandPacket {
    pub action: ClientCommandPacketAction,
}

impl CodablePacket for ClientCommandPacket {
    fn encode(self, buf: &mut BytesMut) {
        buf.set_mc_var_int(self.action as i32);
    }

    fn decode(buf: &mut BytesMut) -> Result<Self> where Self: Sized {
        let action: ClientCommandPacketAction = buf.get_mc_enum()?;
        return Ok(ClientCommandPacket { action });
    }
}
