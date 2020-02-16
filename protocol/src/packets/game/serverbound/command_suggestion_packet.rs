use crate::network::*;
use crate::packet::*;
use crate::result::*;
use bytes::BytesMut;

#[derive(PartialEq, Clone, Debug)]
pub struct CommandSuggestionPacket {
    pub id: i32,
    pub command: String,
}

impl CodablePacket for CommandSuggestionPacket {
    fn encode(self, buf: &mut BytesMut) {
        buf.set_mc_var_int(self.id);
        buf.set_mc_string(self.command);
    }

    fn decode(buf: &mut BytesMut) -> Result<Self>
    where
        Self: Sized,
    {
        let id = buf.get_mc_var_int()?;
        let command = buf.get_mc_string(32500)?;
        return Ok(CommandSuggestionPacket { id, command });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::packet::test::*;

    #[test]
    fn test_cycle() -> Result<()> {
        cycle(CommandSuggestionPacket {
            id: 123345,
            command: "a command".to_string(),
        })
    }
}
