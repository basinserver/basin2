use crate::network::*;
use crate::packet::*;
use crate::result::*;
use bytes::BytesMut;

pub struct PlayerCommandPacket {
    pub id: i32,
    pub action: PlayerCommandPacketAction,
    pub data: i32,
}

impl CodablePacket for PlayerCommandPacket {
    fn encode(self, buf: &mut BytesMut) {
        buf.set_mc_var_int(self.id);
        buf.set_mc_var_int(self.action as i32);
        buf.set_mc_var_int(self.data);
    }

    fn decode(buf: &mut BytesMut) -> Result<Self>
    where
        Self: Sized,
    {
        let id = buf.get_mc_var_int()?;
        let action: PlayerCommandPacketAction = buf.get_mc_enum()?;
        let data = buf.get_mc_var_int()?;
        return Ok(PlayerCommandPacket { id, action, data });
    }
}
