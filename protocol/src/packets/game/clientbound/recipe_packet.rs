
use crate::packet::*;
use crate::network::*;
use bytes::BytesMut;
use crate::result::*;

pub struct RecipePacket {
    pub state: RecipePacketState,
    pub recipes: Vec<String>,
    pub toHighlight: Option<Vec<String>>,
    pub guiOpen: bool,
    pub filteringCraftable: bool,
    pub furnaceGuiOpen: bool,
    pub furnaceFilteringCraftable: bool,
}

impl CodablePacket for RecipePacket {
    fn encode(self, buf: &mut BytesMut) {
        use RecipePacketState::*;

        buf.set_mc_var_int(self.state as i32);
        buf.set_mc_bool(self.guiOpen);
        buf.set_mc_bool(self.filteringCraftable);
        buf.set_mc_bool(self.furnaceGuiOpen);
        buf.set_mc_bool(self.furnaceFilteringCraftable);
        buf.set_mc_var_int(self.recipes.len() as i32);
        for recipe in self.recipes {
            buf.set_mc_string(recipe);
        }
        match self.state {
            Init => {
                buf.set_mc_var_int(self.toHighlight.as_ref().unwrap().len() as i32);
                for highlighting in self.toHighlight.unwrap() {
                    buf.set_mc_string(highlighting);
                }
            }
            _=> (),
        }
    }

    fn decode(buf: &mut BytesMut) -> Result<Self> where Self: Sized {
        use RecipePacketState::*;

        let state: RecipePacketState = buf.get_mc_enum()?;
        let guiOpen = buf.get_mc_bool()?;
        let filteringCraftable = buf.get_mc_bool()?;
        let furnaceGuiOpen = buf.get_mc_bool()?;
        let furnaceFilteringCraftable = buf.get_mc_bool()?;
        let count = buf.get_mc_var_int()?;
        let mut recipes: Vec<String> = vec![];
        for _ in 0..count {
            recipes.push(buf.get_mc_string(32767)?);
        }
        let toHighlight = match state {
            Init => {
                let count = buf.get_mc_var_int()?;
                let mut toHighlight: Vec<String> = vec![];
                for _ in 0..count {
                    toHighlight.push(buf.get_mc_string(32767)?);
                }
                Some(toHighlight)
            },
            _ => None,
        };
        return Ok(RecipePacket { state, recipes, toHighlight, guiOpen, filteringCraftable, furnaceGuiOpen, furnaceFilteringCraftable });
    }
}
