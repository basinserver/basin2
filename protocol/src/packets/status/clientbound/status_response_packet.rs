
use crate::packet::*;
use crate::network::*;
use bytes::BytesMut;
use uuid::Uuid;
use crate::result::*;

pub struct StatusResponsePacket {
    pub status: ServerStatus,
}

impl CodablePacket for StatusResponsePacket {
    fn encode(self, buf: &mut BytesMut) {
        // TODO: UNKNOWN: var1.writeUtf(GSON.toJson((Object)this.status));
    }

    fn decode(buf: &mut BytesMut) -> Result<Self> where Self: Sized {
        // TODO: UNKNOWN: this.status = (ServerStatus)GsonHelper.fromJson(GSON, var1.readUtf(32767), ServerStatus.class);
        return Ok(StatusResponsePacket { status });
    }
}
