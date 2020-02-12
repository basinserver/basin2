
use crate::packet::*;
use crate::network::*;
use bytes::BytesMut;
use uuid::Uuid;
use crate::result::*;

pub struct ClientCommandPacket {
    pub action: ClientCommandPacketAction,
}

impl CodablePacket for ClientCommandPacket {
    fn encode(self, buf: &mut BytesMut) {
        buf.set_mc_var_int(self.action);
    }

    fn decode(buf: &mut BytesMut) -> Result<Self> where Self: Sized {
        // TODO: UNKNOWN: this.action = (ServerboundClientCommandPacket.Action)var1.readEnum(ServerboundClientCommandPacket.Action.class);
        return Ok(ClientCommandPacket { action });
    }
}
