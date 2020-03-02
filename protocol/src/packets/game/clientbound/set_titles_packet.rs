use crate::network::*;
use crate::packet::*;
use basin2_lib::result::*;
use bytes::BytesMut;

#[derive(PartialEq, Clone, Debug)]
pub struct SetTitlesPacket {
    pub titleType: SetTitlesPacketType,
    pub text: Option<ChatComponent>,
    pub fadeInTime: Option<i32>,
    pub stayTime: Option<i32>,
    pub fadeOutTime: Option<i32>,
}

impl CodablePacket for SetTitlesPacket {
    fn encode(self, buf: &mut BytesMut) {
        use SetTitlesPacketType::*;

        buf.set_mc_var_int(self.titleType as i32);
        match self.titleType {
            Title => {
                buf.set_mc_chat_component(self.text.unwrap());
            }
            Subtitle => {
                buf.set_mc_chat_component(self.text.unwrap());
            }
            Actionbar => {
                buf.set_mc_chat_component(self.text.unwrap());
            }
            Times => {
                buf.set_mc_i32(self.fadeInTime.unwrap());
                buf.set_mc_i32(self.stayTime.unwrap());
                buf.set_mc_i32(self.fadeOutTime.unwrap());
            }
            _ => (),
        }
    }

    fn decode(buf: &mut BytesMut) -> Result<Self>
    where
        Self: Sized,
    {
        use SetTitlesPacketType::*;

        let titleType: SetTitlesPacketType = buf.get_mc_enum()?;
        let (text, fadeInTime, stayTime, fadeOutTime) = match titleType {
            Title => (Some(buf.get_mc_chat_component()?), None, None, None),
            Subtitle => (Some(buf.get_mc_chat_component()?), None, None, None),
            Actionbar => (Some(buf.get_mc_chat_component()?), None, None, None),
            Times => (
                None,
                Some(buf.get_mc_i32()?),
                Some(buf.get_mc_i32()?),
                Some(buf.get_mc_i32()?),
            ),
            _ => (None, None, None, None),
        };
        return Ok(SetTitlesPacket {
            titleType,
            text,
            fadeInTime,
            stayTime,
            fadeOutTime,
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::packet::test::*;

    #[test]
    fn test_cycle_title() -> Result<()> {
        cycle(SetTitlesPacket {
            titleType: SetTitlesPacketType::Title,
            text: Some(ChatComponent::from("test title".to_string())),
            fadeInTime: None,
            stayTime: None,
            fadeOutTime: None,
        })
    }

    #[test]
    fn test_cycle_times() -> Result<()> {
        cycle(SetTitlesPacket {
            titleType: SetTitlesPacketType::Times,
            text: None,
            fadeInTime: Some(30),
            stayTime: Some(20),
            fadeOutTime: Some(50),
        })
    }
}
