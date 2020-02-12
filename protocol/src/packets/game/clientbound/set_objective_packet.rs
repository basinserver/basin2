
use crate::packet::*;
use crate::network::*;
use bytes::BytesMut;
use uuid::Uuid;
use crate::result::*;

pub struct SetObjectivePacket {
    pub objectiveName: String,
    pub displayName: ChatComponent,
    pub renderType: ObjectiveCriteriaRenderType,
    pub method: i32,
}

impl CodablePacket for SetObjectivePacket {
    fn encode(self, buf: &mut BytesMut) {
        /* TODO: NOT FOUND */
    }

    fn decode(buf: &mut BytesMut) -> Result<Self> where Self: Sized {
        /* TODO: NOT FOUND */
        return Ok(SetObjectivePacket { objectiveName, displayName, renderType, method });
    }
}
