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

use bytes::BytesMut;
use crate::Result;
use std::io::Error as IoError;
use std::io::ErrorKind;
use crate::packet::*;
use crate::network::*;

pub fn decode_packet(id: i32, buf: &mut BytesMut) -> Result<PacketGameServerbound> {
    match id {
        0 => Ok(PacketGameServerbound::AcceptTeleportationPacket(AcceptTeleportationPacket::decode(buf)?)),
        1 => Ok(PacketGameServerbound::BlockEntityTagQuery(BlockEntityTagQuery::decode(buf)?)),
        2 => Ok(PacketGameServerbound::ChangeDifficultyPacket(ChangeDifficultyPacket::decode(buf)?)),
        3 => Ok(PacketGameServerbound::ChatPacket(ChatPacket::decode(buf)?)),
        4 => Ok(PacketGameServerbound::ClientCommandPacket(ClientCommandPacket::decode(buf)?)),
        5 => Ok(PacketGameServerbound::ClientInformationPacket(ClientInformationPacket::decode(buf)?)),
        6 => Ok(PacketGameServerbound::CommandSuggestionPacket(CommandSuggestionPacket::decode(buf)?)),
        7 => Ok(PacketGameServerbound::ContainerAckPacket(ContainerAckPacket::decode(buf)?)),
        8 => Ok(PacketGameServerbound::ContainerButtonClickPacket(ContainerButtonClickPacket::decode(buf)?)),
        9 => Ok(PacketGameServerbound::ContainerClickPacket(ContainerClickPacket::decode(buf)?)),
        10 => Ok(PacketGameServerbound::ContainerClosePacket(ContainerClosePacket::decode(buf)?)),
        11 => Ok(PacketGameServerbound::CustomPayloadPacket(CustomPayloadPacket::decode(buf)?)),
        12 => Ok(PacketGameServerbound::EditBookPacket(EditBookPacket::decode(buf)?)),
        13 => Ok(PacketGameServerbound::EntityTagQuery(EntityTagQuery::decode(buf)?)),
        14 => Ok(PacketGameServerbound::InteractPacket(InteractPacket::decode(buf)?)),
        15 => Ok(PacketGameServerbound::KeepAlivePacket(KeepAlivePacket::decode(buf)?)),
        16 => Ok(PacketGameServerbound::LockDifficultyPacket(LockDifficultyPacket::decode(buf)?)),
        17 => Err(/* TODO: NYI */)
        18 => Err(/* TODO: NYI */)
        19 => Err(/* TODO: NYI */)
        20 => Ok(PacketGameServerbound::MovePlayerPacket(MovePlayerPacket::decode(buf)?)),
        21 => Ok(PacketGameServerbound::MoveVehiclePacket(MoveVehiclePacket::decode(buf)?)),
        22 => Ok(PacketGameServerbound::PaddleBoatPacket(PaddleBoatPacket::decode(buf)?)),
        23 => Ok(PacketGameServerbound::PickItemPacket(PickItemPacket::decode(buf)?)),
        24 => Ok(PacketGameServerbound::PlaceRecipePacket(PlaceRecipePacket::decode(buf)?)),
        25 => Ok(PacketGameServerbound::PlayerAbilitiesPacket(PlayerAbilitiesPacket::decode(buf)?)),
        26 => Ok(PacketGameServerbound::PlayerActionPacket(PlayerActionPacket::decode(buf)?)),
        27 => Ok(PacketGameServerbound::PlayerCommandPacket(PlayerCommandPacket::decode(buf)?)),
        28 => Ok(PacketGameServerbound::PlayerInputPacket(PlayerInputPacket::decode(buf)?)),
        29 => Ok(PacketGameServerbound::RecipeBookUpdatePacket(RecipeBookUpdatePacket::decode(buf)?)),
        30 => Ok(PacketGameServerbound::RenameItemPacket(RenameItemPacket::decode(buf)?)),
        31 => Ok(PacketGameServerbound::ResourcePackPacket(ResourcePackPacket::decode(buf)?)),
        32 => Ok(PacketGameServerbound::SeenAdvancementsPacket(SeenAdvancementsPacket::decode(buf)?)),
        33 => Ok(PacketGameServerbound::SelectTradePacket(SelectTradePacket::decode(buf)?)),
        34 => Ok(PacketGameServerbound::SetBeaconPacket(SetBeaconPacket::decode(buf)?)),
        35 => Ok(PacketGameServerbound::SetCarriedItemPacket(SetCarriedItemPacket::decode(buf)?)),
        36 => Ok(PacketGameServerbound::SetCommandBlockPacket(SetCommandBlockPacket::decode(buf)?)),
        37 => Ok(PacketGameServerbound::SetCommandMinecartPacket(SetCommandMinecartPacket::decode(buf)?)),
        38 => Ok(PacketGameServerbound::SetCreativeModeSlotPacket(SetCreativeModeSlotPacket::decode(buf)?)),
        39 => Ok(PacketGameServerbound::SetJigsawBlockPacket(SetJigsawBlockPacket::decode(buf)?)),
        40 => Ok(PacketGameServerbound::SetStructureBlockPacket(SetStructureBlockPacket::decode(buf)?)),
        41 => Ok(PacketGameServerbound::SignUpdatePacket(SignUpdatePacket::decode(buf)?)),
        42 => Ok(PacketGameServerbound::SwingPacket(SwingPacket::decode(buf)?)),
        43 => Ok(PacketGameServerbound::TeleportToEntityPacket(TeleportToEntityPacket::decode(buf)?)),
        44 => Ok(PacketGameServerbound::UseItemOnPacket(UseItemOnPacket::decode(buf)?)),
        45 => Ok(PacketGameServerbound::UseItemPacket(UseItemPacket::decode(buf)?)),
        _ => Err(Box::new(IoError::from(ErrorKind::InvalidData)))
    }
}

pub fn encode_packet(packet: PacketGameServerbound, buf: &mut BytesMut) {
    match packet {
        PacketGameServerbound::AcceptTeleportationPacket(deref_packet) => { buf.set_mc_var_int(0); deref_packet.encode(buf); },
        PacketGameServerbound::BlockEntityTagQuery(deref_packet) => { buf.set_mc_var_int(1); deref_packet.encode(buf); },
        PacketGameServerbound::ChangeDifficultyPacket(deref_packet) => { buf.set_mc_var_int(2); deref_packet.encode(buf); },
        PacketGameServerbound::ChatPacket(deref_packet) => { buf.set_mc_var_int(3); deref_packet.encode(buf); },
        PacketGameServerbound::ClientCommandPacket(deref_packet) => { buf.set_mc_var_int(4); deref_packet.encode(buf); },
        PacketGameServerbound::ClientInformationPacket(deref_packet) => { buf.set_mc_var_int(5); deref_packet.encode(buf); },
        PacketGameServerbound::CommandSuggestionPacket(deref_packet) => { buf.set_mc_var_int(6); deref_packet.encode(buf); },
        PacketGameServerbound::ContainerAckPacket(deref_packet) => { buf.set_mc_var_int(7); deref_packet.encode(buf); },
        PacketGameServerbound::ContainerButtonClickPacket(deref_packet) => { buf.set_mc_var_int(8); deref_packet.encode(buf); },
        PacketGameServerbound::ContainerClickPacket(deref_packet) => { buf.set_mc_var_int(9); deref_packet.encode(buf); },
        PacketGameServerbound::ContainerClosePacket(deref_packet) => { buf.set_mc_var_int(10); deref_packet.encode(buf); },
        PacketGameServerbound::CustomPayloadPacket(deref_packet) => { buf.set_mc_var_int(11); deref_packet.encode(buf); },
        PacketGameServerbound::EditBookPacket(deref_packet) => { buf.set_mc_var_int(12); deref_packet.encode(buf); },
        PacketGameServerbound::EntityTagQuery(deref_packet) => { buf.set_mc_var_int(13); deref_packet.encode(buf); },
        PacketGameServerbound::InteractPacket(deref_packet) => { buf.set_mc_var_int(14); deref_packet.encode(buf); },
        PacketGameServerbound::KeepAlivePacket(deref_packet) => { buf.set_mc_var_int(15); deref_packet.encode(buf); },
        PacketGameServerbound::LockDifficultyPacket(deref_packet) => { buf.set_mc_var_int(16); deref_packet.encode(buf); },
        null => Err(/* TODO: NYI */)
        null => Err(/* TODO: NYI */)
        null => Err(/* TODO: NYI */)
        PacketGameServerbound::MovePlayerPacket(deref_packet) => { buf.set_mc_var_int(20); deref_packet.encode(buf); },
        PacketGameServerbound::MoveVehiclePacket(deref_packet) => { buf.set_mc_var_int(21); deref_packet.encode(buf); },
        PacketGameServerbound::PaddleBoatPacket(deref_packet) => { buf.set_mc_var_int(22); deref_packet.encode(buf); },
        PacketGameServerbound::PickItemPacket(deref_packet) => { buf.set_mc_var_int(23); deref_packet.encode(buf); },
        PacketGameServerbound::PlaceRecipePacket(deref_packet) => { buf.set_mc_var_int(24); deref_packet.encode(buf); },
        PacketGameServerbound::PlayerAbilitiesPacket(deref_packet) => { buf.set_mc_var_int(25); deref_packet.encode(buf); },
        PacketGameServerbound::PlayerActionPacket(deref_packet) => { buf.set_mc_var_int(26); deref_packet.encode(buf); },
        PacketGameServerbound::PlayerCommandPacket(deref_packet) => { buf.set_mc_var_int(27); deref_packet.encode(buf); },
        PacketGameServerbound::PlayerInputPacket(deref_packet) => { buf.set_mc_var_int(28); deref_packet.encode(buf); },
        PacketGameServerbound::RecipeBookUpdatePacket(deref_packet) => { buf.set_mc_var_int(29); deref_packet.encode(buf); },
        PacketGameServerbound::RenameItemPacket(deref_packet) => { buf.set_mc_var_int(30); deref_packet.encode(buf); },
        PacketGameServerbound::ResourcePackPacket(deref_packet) => { buf.set_mc_var_int(31); deref_packet.encode(buf); },
        PacketGameServerbound::SeenAdvancementsPacket(deref_packet) => { buf.set_mc_var_int(32); deref_packet.encode(buf); },
        PacketGameServerbound::SelectTradePacket(deref_packet) => { buf.set_mc_var_int(33); deref_packet.encode(buf); },
        PacketGameServerbound::SetBeaconPacket(deref_packet) => { buf.set_mc_var_int(34); deref_packet.encode(buf); },
        PacketGameServerbound::SetCarriedItemPacket(deref_packet) => { buf.set_mc_var_int(35); deref_packet.encode(buf); },
        PacketGameServerbound::SetCommandBlockPacket(deref_packet) => { buf.set_mc_var_int(36); deref_packet.encode(buf); },
        PacketGameServerbound::SetCommandMinecartPacket(deref_packet) => { buf.set_mc_var_int(37); deref_packet.encode(buf); },
        PacketGameServerbound::SetCreativeModeSlotPacket(deref_packet) => { buf.set_mc_var_int(38); deref_packet.encode(buf); },
        PacketGameServerbound::SetJigsawBlockPacket(deref_packet) => { buf.set_mc_var_int(39); deref_packet.encode(buf); },
        PacketGameServerbound::SetStructureBlockPacket(deref_packet) => { buf.set_mc_var_int(40); deref_packet.encode(buf); },
        PacketGameServerbound::SignUpdatePacket(deref_packet) => { buf.set_mc_var_int(41); deref_packet.encode(buf); },
        PacketGameServerbound::SwingPacket(deref_packet) => { buf.set_mc_var_int(42); deref_packet.encode(buf); },
        PacketGameServerbound::TeleportToEntityPacket(deref_packet) => { buf.set_mc_var_int(43); deref_packet.encode(buf); },
        PacketGameServerbound::UseItemOnPacket(deref_packet) => { buf.set_mc_var_int(44); deref_packet.encode(buf); },
        PacketGameServerbound::UseItemPacket(deref_packet) => { buf.set_mc_var_int(45); deref_packet.encode(buf); },
    }
}

pub enum PacketGameServerbound {
    AcceptTeleportationPacket(AcceptTeleportationPacket),
    BlockEntityTagQuery(BlockEntityTagQuery),
    ChangeDifficultyPacket(ChangeDifficultyPacket),
    ChatPacket(ChatPacket),
    ClientCommandPacket(ClientCommandPacket),
    ClientInformationPacket(ClientInformationPacket),
    CommandSuggestionPacket(CommandSuggestionPacket),
    ContainerAckPacket(ContainerAckPacket),
    ContainerButtonClickPacket(ContainerButtonClickPacket),
    ContainerClickPacket(ContainerClickPacket),
    ContainerClosePacket(ContainerClosePacket),
    CustomPayloadPacket(CustomPayloadPacket),
    EditBookPacket(EditBookPacket),
    EntityTagQuery(EntityTagQuery),
    InteractPacket(InteractPacket),
    KeepAlivePacket(KeepAlivePacket),
    LockDifficultyPacket(LockDifficultyPacket),
    /* TODO: NYI */
    /* TODO: NYI */
    /* TODO: NYI */
    MovePlayerPacket(MovePlayerPacket),
    MoveVehiclePacket(MoveVehiclePacket),
    PaddleBoatPacket(PaddleBoatPacket),
    PickItemPacket(PickItemPacket),
    PlaceRecipePacket(PlaceRecipePacket),
    PlayerAbilitiesPacket(PlayerAbilitiesPacket),
    PlayerActionPacket(PlayerActionPacket),
    PlayerCommandPacket(PlayerCommandPacket),
    PlayerInputPacket(PlayerInputPacket),
    RecipeBookUpdatePacket(RecipeBookUpdatePacket),
    RenameItemPacket(RenameItemPacket),
    ResourcePackPacket(ResourcePackPacket),
    SeenAdvancementsPacket(SeenAdvancementsPacket),
    SelectTradePacket(SelectTradePacket),
    SetBeaconPacket(SetBeaconPacket),
    SetCarriedItemPacket(SetCarriedItemPacket),
    SetCommandBlockPacket(SetCommandBlockPacket),
    SetCommandMinecartPacket(SetCommandMinecartPacket),
    SetCreativeModeSlotPacket(SetCreativeModeSlotPacket),
    SetJigsawBlockPacket(SetJigsawBlockPacket),
    SetStructureBlockPacket(SetStructureBlockPacket),
    SignUpdatePacket(SignUpdatePacket),
    SwingPacket(SwingPacket),
    TeleportToEntityPacket(TeleportToEntityPacket),
    UseItemOnPacket(UseItemOnPacket),
    UseItemPacket(UseItemPacket),
}

