use crate::network::*;
use crate::packet::*;
use crate::result::*;
use bytes::BytesMut;

pub struct SetScorePacket {
    pub owner: String,
    pub objectiveName: String,
    pub score: i32,
    pub method: ServerScoreboardMethod,
}

impl CodablePacket for SetScorePacket {
    fn encode(self, buf: &mut BytesMut) {
        use ServerScoreboardMethod::*;

        buf.set_mc_string(self.owner);
        buf.set_mc_var_int(self.method as i32);
        buf.set_mc_string(self.objectiveName);
        match self.method {
            Remove => (),
            _ => buf.set_mc_var_int(self.score),
        }
    }

    fn decode(buf: &mut BytesMut) -> Result<Self>
    where
        Self: Sized,
    {
        use ServerScoreboardMethod::*;

        let owner = buf.get_mc_string(40)?;
        let method: ServerScoreboardMethod = buf.get_mc_enum()?;
        let objectiveName = buf.get_mc_string(16)?;
        let score = match method {
            Remove => 0,
            _ => buf.get_mc_var_int()?,
        };
        return Ok(SetScorePacket {
            owner,
            objectiveName,
            score,
            method,
        });
    }
}
