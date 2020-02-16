use crate::network::*;
use crate::packet::*;
use crate::result::*;
use bytes::BytesMut;

#[derive(PartialEq, Clone, Debug)]
pub struct SetExperiencePacket {
    pub experienceProgress: f32,
    pub totalExperience: i32,
    pub experienceLevel: i32,
}

impl CodablePacket for SetExperiencePacket {
    fn encode(self, buf: &mut BytesMut) {
        buf.set_mc_f32(self.experienceProgress);
        buf.set_mc_var_int(self.experienceLevel);
        buf.set_mc_var_int(self.totalExperience);
    }

    fn decode(buf: &mut BytesMut) -> Result<Self>
    where
        Self: Sized,
    {
        let experienceProgress = buf.get_mc_f32()?;
        let experienceLevel = buf.get_mc_var_int()?;
        let totalExperience = buf.get_mc_var_int()?;
        return Ok(SetExperiencePacket {
            experienceProgress,
            totalExperience,
            experienceLevel,
        });
    }
}
