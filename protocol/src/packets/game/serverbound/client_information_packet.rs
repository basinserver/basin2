
use crate::packet::*;
use crate::network::*;
use bytes::BytesMut;
use uuid::Uuid;
use crate::result::*;

pub struct ClientInformationPacket {
    pub language: String,
    pub viewDistance: i32,
    pub chatVisibility: ChatVisiblity,
    pub chatColors: bool,
    pub modelCustomisation: i32,
    pub mainHand: HumanoidArm,
}

impl CodablePacket for ClientInformationPacket {
    fn encode(self, buf: &mut BytesMut) {
        buf.set_mc_string(self.language);
        buf.set_mc_u8(self.viewDistance);
        buf.set_mc_var_int(self.chatVisibility);
        buf.set_mc_bool(self.chatColors);
        buf.set_mc_u8(self.modelCustomisation);
        buf.set_mc_var_int(self.mainHand);
    }

    fn decode(buf: &mut BytesMut) -> Result<Self> where Self: Sized {
        let language = buf.get_mc_string_bounded(16)?;
        let viewDistance = buf.get_mc_u8()?;
        // TODO: UNKNOWN: this.chatVisibility = (ChatVisiblity)var1.readEnum(ChatVisiblity.class);
        let chatColors = buf.get_mc_bool()?;
        let modelCustomisation = buf.get_mc_u8()?;
        // TODO: UNKNOWN: this.mainHand = (HumanoidArm)var1.readEnum(HumanoidArm.class);
        return Ok(ClientInformationPacket { language, viewDistance, chatVisibility, chatColors, modelCustomisation, mainHand });
    }
}
