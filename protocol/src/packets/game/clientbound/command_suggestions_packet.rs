use crate::network::*;
use crate::packet::*;
use basin2_lib::result::*;
use bytes::BytesMut;

#[derive(PartialEq, Clone, Debug)]
pub struct CommandSuggestionsPacket {
    pub id: i32,
    pub suggestions: Suggestions,
}

impl CodablePacket for CommandSuggestionsPacket {
    fn encode(self, buf: &mut BytesMut) {
        buf.set_mc_var_int(self.id);
        buf.set_mc_var_int(self.suggestions.start);
        buf.set_mc_var_int(self.suggestions.end - self.suggestions.start);
        buf.set_mc_var_int(self.suggestions.suggestions.len() as i32);
        for suggestion in self.suggestions.suggestions {
            buf.set_mc_string(suggestion.text);
            match suggestion.tooltip {
                Some(tooltip) => {
                    buf.set_mc_bool(true);
                    buf.set_mc_chat_component(tooltip);
                }
                None => {
                    buf.set_mc_bool(false);
                }
            }
        }
    }

    fn decode(buf: &mut BytesMut) -> Result<Self>
    where
        Self: Sized,
    {
        let id = buf.get_mc_var_int()?;
        let start = buf.get_mc_var_int()?;
        let end = start + buf.get_mc_var_int()?;
        let count = buf.get_mc_var_int()?;
        let mut suggestions: Vec<Suggestion> = vec![];
        for _ in 0..count {
            let text = buf.get_mc_string(32767)?;
            let tooltip = if buf.get_mc_bool()? {
                Some(buf.get_mc_chat_component()?)
            } else {
                None
            };
            suggestions.push(Suggestion { text, tooltip })
        }
        return Ok(CommandSuggestionsPacket {
            id,
            suggestions: Suggestions {
                start,
                end,
                suggestions,
            },
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::packet::test::*;

    #[test]
    fn test_cycle() -> Result<()> {
        cycle(CommandSuggestionsPacket {
            id: 4321,
            suggestions: Suggestions {
                start: 30,
                end: 40,
                suggestions: vec![Suggestion {
                    text: "try something else".to_string(),
                    tooltip: Some(ChatComponent::from("tooltip".to_string())),
                }],
            },
        })
    }
}
