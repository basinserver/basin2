use crate::network::*;
use crate::packet::*;
use crate::result::*;
use bytes::BytesMut;

pub enum RecipeBookUpdatePacketData {
    Shown(ResourceLocation),
    Settings(bool, bool, bool, bool, bool, bool, bool, bool),
}

pub struct RecipeBookUpdatePacket {
    pub purpose: RecipeBookUpdatePacketPurpose,
    pub data: RecipeBookUpdatePacketData,
}

impl CodablePacket for RecipeBookUpdatePacket {
    fn encode(self, buf: &mut BytesMut) {
        buf.set_mc_var_int(self.purpose as i32);
        match (self.purpose, self.data) {
            (RecipeBookUpdatePacketPurpose::Shown, RecipeBookUpdatePacketData::Shown(recipe)) => {
                buf.set_mc_string(recipe);
            }
            (
                RecipeBookUpdatePacketPurpose::Settings,
                RecipeBookUpdatePacketData::Settings(
                    guiOpen,
                    filteringCraftable,
                    furnaceGuiOpen,
                    furnaceFilteringCraftable,
                    blastFurnaceGuiOpen,
                    blastFurnaceFilteringCraftable,
                    smokerGuiOpen,
                    smokerFilteringCraftable,
                ),
            ) => {
                buf.set_mc_bool(guiOpen);
                buf.set_mc_bool(filteringCraftable);
                buf.set_mc_bool(furnaceGuiOpen);
                buf.set_mc_bool(furnaceFilteringCraftable);
                buf.set_mc_bool(blastFurnaceGuiOpen);
                buf.set_mc_bool(blastFurnaceFilteringCraftable);
                buf.set_mc_bool(smokerGuiOpen);
                buf.set_mc_bool(smokerFilteringCraftable);
            }
            _ => panic!("invalid formed outgoing recipe_book_update_packet, mismatched types"),
        }
    }

    fn decode(buf: &mut BytesMut) -> Result<Self>
    where
        Self: Sized,
    {
        use RecipeBookUpdatePacketPurpose::*;
        let purpose: RecipeBookUpdatePacketPurpose = buf.get_mc_enum()?;
        let data = match purpose {
            Shown => RecipeBookUpdatePacketData::Shown(buf.get_mc_string(32767)?),
            Settings => RecipeBookUpdatePacketData::Settings(
                buf.get_mc_bool()?,
                buf.get_mc_bool()?,
                buf.get_mc_bool()?,
                buf.get_mc_bool()?,
                buf.get_mc_bool()?,
                buf.get_mc_bool()?,
                buf.get_mc_bool()?,
                buf.get_mc_bool()?,
            ),
        };
        return Ok(RecipeBookUpdatePacket { purpose, data });
    }
}
