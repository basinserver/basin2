
use crate::packet::*;
use crate::network::*;
use bytes::BytesMut;
use uuid::Uuid;
use crate::result::*;

pub struct HelloPacket {
    pub gameProfile: GameProfile,
}

impl CodablePacket for HelloPacket {
    fn encode(self, buf: &mut BytesMut) {
        // TODO: UNKNOWN: var1.writeUtf(this.gameProfile.getName());
    }

    fn decode(buf: &mut BytesMut) -> Result<Self> where Self: Sized {
        // TODO: UNKNOWN: this.gameProfile = new GameProfile((UUID)null, var1.readUtf(16));
        return Ok(HelloPacket { gameProfile });
    }
}
