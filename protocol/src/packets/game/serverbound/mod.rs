pub mod accept_teleportation_packet;
pub use accept_teleportation_packet::*;
pub mod block_entity_tag_query;
pub use block_entity_tag_query::*;
pub mod change_difficulty_packet;
pub use change_difficulty_packet::*;
pub mod chat_packet;
pub use chat_packet::*;
pub mod client_command_packet;
pub use client_command_packet::*;
pub mod client_information_packet;
pub use client_information_packet::*;
pub mod command_suggestion_packet;
pub use command_suggestion_packet::*;
pub mod container_ack_packet;
pub use container_ack_packet::*;
pub mod container_button_click_packet;
pub use container_button_click_packet::*;
pub mod container_click_packet;
pub use container_click_packet::*;
pub mod container_close_packet;
pub use container_close_packet::*;
pub mod custom_payload_packet;
pub use custom_payload_packet::*;
pub mod edit_book_packet;
pub use edit_book_packet::*;
pub mod entity_tag_query;
pub use entity_tag_query::*;
pub mod interact_packet;
pub use interact_packet::*;
pub mod keep_alive_packet;
pub use keep_alive_packet::*;
pub mod lock_difficulty_packet;
pub use lock_difficulty_packet::*;
pub mod move_player_pos_packet;
pub use move_player_pos_packet::*;
pub mod move_player_posrot_packet;
pub use move_player_posrot_packet::*;
pub mod move_player_rot_packet;
pub use move_player_rot_packet::*;
pub mod move_player_packet;
pub use move_player_packet::*;
pub mod move_vehicle_packet;
pub use move_vehicle_packet::*;
pub mod paddle_boat_packet;
pub use paddle_boat_packet::*;
pub mod pick_item_packet;
pub use pick_item_packet::*;
pub mod place_recipe_packet;
pub use place_recipe_packet::*;
pub mod player_abilities_packet;
pub use player_abilities_packet::*;
pub mod player_action_packet;
pub use player_action_packet::*;
pub mod player_command_packet;
pub use player_command_packet::*;
pub mod player_input_packet;
pub use player_input_packet::*;
pub mod recipe_book_update_packet;
pub use recipe_book_update_packet::*;
pub mod rename_item_packet;
pub use rename_item_packet::*;
pub mod resource_pack_packet;
pub use resource_pack_packet::*;
pub mod seen_advancements_packet;
pub use seen_advancements_packet::*;
pub mod select_trade_packet;
pub use select_trade_packet::*;
pub mod set_beacon_packet;
pub use set_beacon_packet::*;
pub mod set_carried_item_packet;
pub use set_carried_item_packet::*;
pub mod set_command_block_packet;
pub use set_command_block_packet::*;
pub mod set_command_minecart_packet;
pub use set_command_minecart_packet::*;
pub mod set_creative_mode_slot_packet;
pub use set_creative_mode_slot_packet::*;
pub mod set_jigsaw_block_packet;
pub use set_jigsaw_block_packet::*;
pub mod set_structure_block_packet;
pub use set_structure_block_packet::*;
pub mod sign_update_packet;
pub use sign_update_packet::*;
pub mod swing_packet;
pub use swing_packet::*;
pub mod teleport_to_entity_packet;
pub use teleport_to_entity_packet::*;
pub mod use_item_on_packet;
pub use use_item_on_packet::*;
pub mod use_item_packet;
pub use use_item_packet::*;

use super::Game;
use crate::network::*;
use crate::packet::*;
use crate::Result;
use bytes::BytesMut;
use std::io::Error as IoError;
use std::io::ErrorKind;

hierarchy! {
    child<Game> enum GameServerbound {
        AcceptTeleportationPacket,
        BlockEntityTagQuery,
        ChangeDifficultyPacket,
        ChatPacket,
        ClientCommandPacket,
        ClientInformationPacket,
        CommandSuggestionPacket,
        ContainerAckPacket,
        ContainerButtonClickPacket,
        ContainerClickPacket,
        ContainerClosePacket,
        CustomPayloadPacket,
        EditBookPacket,
        EntityTagQuery,
        InteractPacket,
        KeepAlivePacket,
        LockDifficultyPacket,
        MovePlayerPosPacket,
        MovePlayerPosRotPacket,
        MovePlayerRotPacket,
        MovePlayerPacket,
        MoveVehiclePacket,
        PaddleBoatPacket,
        PickItemPacket,
        PlaceRecipePacket,
        PlayerAbilitiesPacket,
        PlayerActionPacket,
        PlayerCommandPacket,
        PlayerInputPacket,
        RecipeBookUpdatePacket,
        RenameItemPacket,
        ResourcePackPacket,
        SeenAdvancementsPacket,
        SelectTradePacket,
        SetBeaconPacket,
        SetCarriedItemPacket,
        SetCommandBlockPacket,
        SetCommandMinecartPacket,
        SetCreativeModeSlotPacket,
        SetJigsawBlockPacket,
        SetStructureBlockPacket,
        SignUpdatePacket,
        SwingPacket,
        TeleportToEntityPacket,
        UseItemOnPacket,
        UseItemPacket,
    }
}

impl PacketContainer for GameServerbound {
    fn encode(self, buf: &mut BytesMut) {
        use GameServerbound::*;
        match self {
            AcceptTeleportationPacket(deref_packet) => {
                buf.set_mc_var_int(0);
                deref_packet.encode(buf);
            }
            BlockEntityTagQuery(deref_packet) => {
                buf.set_mc_var_int(1);
                deref_packet.encode(buf);
            }
            ChangeDifficultyPacket(deref_packet) => {
                buf.set_mc_var_int(2);
                deref_packet.encode(buf);
            }
            ChatPacket(deref_packet) => {
                buf.set_mc_var_int(3);
                deref_packet.encode(buf);
            }
            ClientCommandPacket(deref_packet) => {
                buf.set_mc_var_int(4);
                deref_packet.encode(buf);
            }
            ClientInformationPacket(deref_packet) => {
                buf.set_mc_var_int(5);
                deref_packet.encode(buf);
            }
            CommandSuggestionPacket(deref_packet) => {
                buf.set_mc_var_int(6);
                deref_packet.encode(buf);
            }
            ContainerAckPacket(deref_packet) => {
                buf.set_mc_var_int(7);
                deref_packet.encode(buf);
            }
            ContainerButtonClickPacket(deref_packet) => {
                buf.set_mc_var_int(8);
                deref_packet.encode(buf);
            }
            ContainerClickPacket(deref_packet) => {
                buf.set_mc_var_int(9);
                deref_packet.encode(buf);
            }
            ContainerClosePacket(deref_packet) => {
                buf.set_mc_var_int(10);
                deref_packet.encode(buf);
            }
            CustomPayloadPacket(deref_packet) => {
                buf.set_mc_var_int(11);
                deref_packet.encode(buf);
            }
            EditBookPacket(deref_packet) => {
                buf.set_mc_var_int(12);
                deref_packet.encode(buf);
            }
            EntityTagQuery(deref_packet) => {
                buf.set_mc_var_int(13);
                deref_packet.encode(buf);
            }
            InteractPacket(deref_packet) => {
                buf.set_mc_var_int(14);
                deref_packet.encode(buf);
            }
            KeepAlivePacket(deref_packet) => {
                buf.set_mc_var_int(15);
                deref_packet.encode(buf);
            }
            LockDifficultyPacket(deref_packet) => {
                buf.set_mc_var_int(16);
                deref_packet.encode(buf);
            }
            MovePlayerPosPacket(deref_packet) => {
                buf.set_mc_var_int(17);
                deref_packet.encode(buf);
            }
            MovePlayerPosRotPacket(deref_packet) => {
                buf.set_mc_var_int(18);
                deref_packet.encode(buf);
            }
            MovePlayerRotPacket(deref_packet) => {
                buf.set_mc_var_int(19);
                deref_packet.encode(buf);
            }
            MovePlayerPacket(deref_packet) => {
                buf.set_mc_var_int(20);
                deref_packet.encode(buf);
            }
            MoveVehiclePacket(deref_packet) => {
                buf.set_mc_var_int(21);
                deref_packet.encode(buf);
            }
            PaddleBoatPacket(deref_packet) => {
                buf.set_mc_var_int(22);
                deref_packet.encode(buf);
            }
            PickItemPacket(deref_packet) => {
                buf.set_mc_var_int(23);
                deref_packet.encode(buf);
            }
            PlaceRecipePacket(deref_packet) => {
                buf.set_mc_var_int(24);
                deref_packet.encode(buf);
            }
            PlayerAbilitiesPacket(deref_packet) => {
                buf.set_mc_var_int(25);
                deref_packet.encode(buf);
            }
            PlayerActionPacket(deref_packet) => {
                buf.set_mc_var_int(26);
                deref_packet.encode(buf);
            }
            PlayerCommandPacket(deref_packet) => {
                buf.set_mc_var_int(27);
                deref_packet.encode(buf);
            }
            PlayerInputPacket(deref_packet) => {
                buf.set_mc_var_int(28);
                deref_packet.encode(buf);
            }
            RecipeBookUpdatePacket(deref_packet) => {
                buf.set_mc_var_int(29);
                deref_packet.encode(buf);
            }
            RenameItemPacket(deref_packet) => {
                buf.set_mc_var_int(30);
                deref_packet.encode(buf);
            }
            ResourcePackPacket(deref_packet) => {
                buf.set_mc_var_int(31);
                deref_packet.encode(buf);
            }
            SeenAdvancementsPacket(deref_packet) => {
                buf.set_mc_var_int(32);
                deref_packet.encode(buf);
            }
            SelectTradePacket(deref_packet) => {
                buf.set_mc_var_int(33);
                deref_packet.encode(buf);
            }
            SetBeaconPacket(deref_packet) => {
                buf.set_mc_var_int(34);
                deref_packet.encode(buf);
            }
            SetCarriedItemPacket(deref_packet) => {
                buf.set_mc_var_int(35);
                deref_packet.encode(buf);
            }
            SetCommandBlockPacket(deref_packet) => {
                buf.set_mc_var_int(36);
                deref_packet.encode(buf);
            }
            SetCommandMinecartPacket(deref_packet) => {
                buf.set_mc_var_int(37);
                deref_packet.encode(buf);
            }
            SetCreativeModeSlotPacket(deref_packet) => {
                buf.set_mc_var_int(38);
                deref_packet.encode(buf);
            }
            SetJigsawBlockPacket(deref_packet) => {
                buf.set_mc_var_int(39);
                deref_packet.encode(buf);
            }
            SetStructureBlockPacket(deref_packet) => {
                buf.set_mc_var_int(40);
                deref_packet.encode(buf);
            }
            SignUpdatePacket(deref_packet) => {
                buf.set_mc_var_int(41);
                deref_packet.encode(buf);
            }
            SwingPacket(deref_packet) => {
                buf.set_mc_var_int(42);
                deref_packet.encode(buf);
            }
            TeleportToEntityPacket(deref_packet) => {
                buf.set_mc_var_int(43);
                deref_packet.encode(buf);
            }
            UseItemOnPacket(deref_packet) => {
                buf.set_mc_var_int(44);
                deref_packet.encode(buf);
            }
            UseItemPacket(deref_packet) => {
                buf.set_mc_var_int(45);
                deref_packet.encode(buf);
            }
        }
    }

    fn decode(id: i32, buf: &mut BytesMut) -> Result<GameServerbound> {
        match id {
            0 => Ok(GameServerbound::AcceptTeleportationPacket(
                AcceptTeleportationPacket::decode(buf)?,
            )),
            1 => Ok(GameServerbound::BlockEntityTagQuery(
                BlockEntityTagQuery::decode(buf)?,
            )),
            2 => Ok(GameServerbound::ChangeDifficultyPacket(
                ChangeDifficultyPacket::decode(buf)?,
            )),
            3 => Ok(GameServerbound::ChatPacket(ChatPacket::decode(buf)?)),
            4 => Ok(GameServerbound::ClientCommandPacket(
                ClientCommandPacket::decode(buf)?,
            )),
            5 => Ok(GameServerbound::ClientInformationPacket(
                ClientInformationPacket::decode(buf)?,
            )),
            6 => Ok(GameServerbound::CommandSuggestionPacket(
                CommandSuggestionPacket::decode(buf)?,
            )),
            7 => Ok(GameServerbound::ContainerAckPacket(
                ContainerAckPacket::decode(buf)?,
            )),
            8 => Ok(GameServerbound::ContainerButtonClickPacket(
                ContainerButtonClickPacket::decode(buf)?,
            )),
            9 => Ok(GameServerbound::ContainerClickPacket(
                ContainerClickPacket::decode(buf)?,
            )),
            10 => Ok(GameServerbound::ContainerClosePacket(
                ContainerClosePacket::decode(buf)?,
            )),
            11 => Ok(GameServerbound::CustomPayloadPacket(
                CustomPayloadPacket::decode(buf)?,
            )),
            12 => Ok(GameServerbound::EditBookPacket(EditBookPacket::decode(
                buf,
            )?)),
            13 => Ok(GameServerbound::EntityTagQuery(EntityTagQuery::decode(
                buf,
            )?)),
            14 => Ok(GameServerbound::InteractPacket(InteractPacket::decode(
                buf,
            )?)),
            15 => Ok(GameServerbound::KeepAlivePacket(KeepAlivePacket::decode(
                buf,
            )?)),
            16 => Ok(GameServerbound::LockDifficultyPacket(
                LockDifficultyPacket::decode(buf)?,
            )),
            17 => Ok(GameServerbound::MovePlayerPosPacket(
                MovePlayerPosPacket::decode(buf)?,
            )),
            18 => Ok(GameServerbound::MovePlayerPosRotPacket(
                MovePlayerPosRotPacket::decode(buf)?,
            )),
            19 => Ok(GameServerbound::MovePlayerRotPacket(
                MovePlayerRotPacket::decode(buf)?,
            )),
            20 => Ok(GameServerbound::MovePlayerPacket(MovePlayerPacket::decode(
                buf,
            )?)),
            21 => Ok(GameServerbound::MoveVehiclePacket(
                MoveVehiclePacket::decode(buf)?,
            )),
            22 => Ok(GameServerbound::PaddleBoatPacket(PaddleBoatPacket::decode(
                buf,
            )?)),
            23 => Ok(GameServerbound::PickItemPacket(PickItemPacket::decode(
                buf,
            )?)),
            24 => Ok(GameServerbound::PlaceRecipePacket(
                PlaceRecipePacket::decode(buf)?,
            )),
            25 => Ok(GameServerbound::PlayerAbilitiesPacket(
                PlayerAbilitiesPacket::decode(buf)?,
            )),
            26 => Ok(GameServerbound::PlayerActionPacket(
                PlayerActionPacket::decode(buf)?,
            )),
            27 => Ok(GameServerbound::PlayerCommandPacket(
                PlayerCommandPacket::decode(buf)?,
            )),
            28 => Ok(GameServerbound::PlayerInputPacket(
                PlayerInputPacket::decode(buf)?,
            )),
            29 => Ok(GameServerbound::RecipeBookUpdatePacket(
                RecipeBookUpdatePacket::decode(buf)?,
            )),
            30 => Ok(GameServerbound::RenameItemPacket(RenameItemPacket::decode(
                buf,
            )?)),
            31 => Ok(GameServerbound::ResourcePackPacket(
                ResourcePackPacket::decode(buf)?,
            )),
            32 => Ok(GameServerbound::SeenAdvancementsPacket(
                SeenAdvancementsPacket::decode(buf)?,
            )),
            33 => Ok(GameServerbound::SelectTradePacket(
                SelectTradePacket::decode(buf)?,
            )),
            34 => Ok(GameServerbound::SetBeaconPacket(SetBeaconPacket::decode(
                buf,
            )?)),
            35 => Ok(GameServerbound::SetCarriedItemPacket(
                SetCarriedItemPacket::decode(buf)?,
            )),
            36 => Ok(GameServerbound::SetCommandBlockPacket(
                SetCommandBlockPacket::decode(buf)?,
            )),
            37 => Ok(GameServerbound::SetCommandMinecartPacket(
                SetCommandMinecartPacket::decode(buf)?,
            )),
            38 => Ok(GameServerbound::SetCreativeModeSlotPacket(
                SetCreativeModeSlotPacket::decode(buf)?,
            )),
            39 => Ok(GameServerbound::SetJigsawBlockPacket(
                SetJigsawBlockPacket::decode(buf)?,
            )),
            40 => Ok(GameServerbound::SetStructureBlockPacket(
                SetStructureBlockPacket::decode(buf)?,
            )),
            41 => Ok(GameServerbound::SignUpdatePacket(SignUpdatePacket::decode(
                buf,
            )?)),
            42 => Ok(GameServerbound::SwingPacket(SwingPacket::decode(buf)?)),
            43 => Ok(GameServerbound::TeleportToEntityPacket(
                TeleportToEntityPacket::decode(buf)?,
            )),
            44 => Ok(GameServerbound::UseItemOnPacket(UseItemOnPacket::decode(
                buf,
            )?)),
            45 => Ok(GameServerbound::UseItemPacket(UseItemPacket::decode(buf)?)),
            _ => Err(Box::new(IoError::from(ErrorKind::InvalidData))),
        }
    }
}
