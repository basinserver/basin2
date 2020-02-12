
use crate::packet::*;
use crate::network::*;
use bytes::BytesMut;
use uuid::Uuid;
use crate::result::*;

pub struct ClientIntentionPacket {
    pub protocolVersion: i32,
    pub hostName: String,
    pub port: i32,
    pub intention: ConnectionProtocol,
}

impl CodablePacket for ClientIntentionPacket {
    fn encode(self, buf: &mut BytesMut) {
        buf.set_mc_var_int(self.protocolVersion);
        buf.set_mc_string(self.hostName);
        buf.set_mc_i16(self.port);
        // TODO: UNKNOWN: var1.writeVarInt(this.intention.getId());
    }

    fn decode(buf: &mut BytesMut) -> Result<Self> where Self: Sized {
        let protocolVersion = buf.get_mc_var_int()?;
        let hostName = buf.get_mc_string_bounded(255)?;
        let port = buf.get_mc_u16()?;
        // TODO: UNKNOWN: this.intention = ConnectionProtocol.getById(var1.readVarInt());
        return Ok(ClientIntentionPacket { protocolVersion, hostName, port, intention });
    }
}
