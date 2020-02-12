
use crate::packet::*;
use crate::network::*;
use bytes::BytesMut;
use uuid::Uuid;
use crate::result::*;

pub struct UpdateTagsPacket {
    pub tags: undefined,
}

impl CodablePacket for UpdateTagsPacket {
    fn encode(self, buf: &mut BytesMut) {
        // TODO: EXTRA: this.tags.serializeToNetwork(var1);
    }

    fn decode(buf: &mut BytesMut) -> Result<Self> where Self: Sized {
        // TODO: EXTRA: this.tags = TagManager.deserializeFromNetwork(var1);
        return Ok(UpdateTagsPacket { tags });
    }
}
