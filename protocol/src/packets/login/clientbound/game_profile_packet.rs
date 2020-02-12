
use crate::packet::*;
use crate::network::*;
use bytes::BytesMut;
use uuid::Uuid;
use crate::result::*;

pub struct GameProfilePacket {
    pub gameProfile: GameProfile,
}

impl CodablePacket for GameProfilePacket {
    fn encode(self, buf: &mut BytesMut) {
        // TODO: EXTRA: UUID var2 = this.gameProfile.getId();
        // TODO: UNKNOWN: var1.writeUtf(var2 == null ? "" : var2.toString());
        // TODO: UNKNOWN: var1.writeUtf(this.gameProfile.getName());
    }

    fn decode(buf: &mut BytesMut) -> Result<Self> where Self: Sized {
        // TODO: UNKNOWN: String var2 = var1.readUtf(36);
        // TODO: UNKNOWN: String var3 = var1.readUtf(16);
        // TODO: EXTRA: UUID var4 = UUID.fromString(var2);
        // TODO: EXTRA: this.gameProfile = new GameProfile(var4, var3);
        return Ok(GameProfilePacket { gameProfile });
    }
}
