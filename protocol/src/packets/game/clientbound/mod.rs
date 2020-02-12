pub mod add_entity_packet;
pub use add_entity_packet::*;
pub mod add_experience_orb_packet;
pub use add_experience_orb_packet::*;
pub mod add_global_entity_packet;
pub use add_global_entity_packet::*;
pub mod add_mob_packet;
pub use add_mob_packet::*;
pub mod add_painting_packet;
pub use add_painting_packet::*;
pub mod add_player_packet;
pub use add_player_packet::*;
pub mod animate_packet;
pub use animate_packet::*;
pub mod award_stats_packet;
pub use award_stats_packet::*;
pub mod block_break_ack_packet;
pub use block_break_ack_packet::*;
pub mod block_destruction_packet;
pub use block_destruction_packet::*;
pub mod block_entity_data_packet;
pub use block_entity_data_packet::*;
pub mod block_event_packet;
pub use block_event_packet::*;
pub mod block_update_packet;
pub use block_update_packet::*;
pub mod boss_event_packet;
pub use boss_event_packet::*;
pub mod change_difficulty_packet;
pub use change_difficulty_packet::*;
pub mod chat_packet;
pub use chat_packet::*;
pub mod chunk_blocks_update_packet;
pub use chunk_blocks_update_packet::*;
pub mod command_suggestions_packet;
pub use command_suggestions_packet::*;
pub mod commands_packet;
pub use commands_packet::*;
pub mod container_ack_packet;
pub use container_ack_packet::*;
pub mod container_close_packet;
pub use container_close_packet::*;
pub mod container_set_content_packet;
pub use container_set_content_packet::*;
pub mod container_set_data_packet;
pub use container_set_data_packet::*;
pub mod container_set_slot_packet;
pub use container_set_slot_packet::*;
pub mod cooldown_packet;
pub use cooldown_packet::*;
pub mod custom_payload_packet;
pub use custom_payload_packet::*;
pub mod custom_sound_packet;
pub use custom_sound_packet::*;
pub mod disconnect_packet;
pub use disconnect_packet::*;
pub mod entity_event_packet;
pub use entity_event_packet::*;
pub mod explode_packet;
pub use explode_packet::*;
pub mod forget_level_chunk_packet;
pub use forget_level_chunk_packet::*;
pub mod game_event_packet;
pub use game_event_packet::*;
pub mod horse_screen_open_packet;
pub use horse_screen_open_packet::*;
pub mod keep_alive_packet;
pub use keep_alive_packet::*;
pub mod level_chunk_packet;
pub use level_chunk_packet::*;
pub mod level_event_packet;
pub use level_event_packet::*;
pub mod level_particles_packet;
pub use level_particles_packet::*;
pub mod light_update_packet;
pub use light_update_packet::*;
pub mod login_packet;
pub use login_packet::*;
pub mod map_item_data_packet;
pub use map_item_data_packet::*;
pub mod merchant_offers_packet;
pub use merchant_offers_packet::*;
pub mod move_entity_packet;
pub use move_entity_packet::*;
pub mod move_vehicle_packet;
pub use move_vehicle_packet::*;
pub mod open_book_packet;
pub use open_book_packet::*;
pub mod open_screen_packet;
pub use open_screen_packet::*;
pub mod open_sign_editor_packet;
pub use open_sign_editor_packet::*;
pub mod place_ghost_recipe_packet;
pub use place_ghost_recipe_packet::*;
pub mod player_abilities_packet;
pub use player_abilities_packet::*;
pub mod player_combat_packet;
pub use player_combat_packet::*;
pub mod player_info_packet;
pub use player_info_packet::*;
pub mod player_look_at_packet;
pub use player_look_at_packet::*;
pub mod player_position_packet;
pub use player_position_packet::*;
pub mod recipe_packet;
pub use recipe_packet::*;
pub mod remove_entities_packet;
pub use remove_entities_packet::*;
pub mod remove_mob_effect_packet;
pub use remove_mob_effect_packet::*;
pub mod resource_pack_packet;
pub use resource_pack_packet::*;
pub mod respawn_packet;
pub use respawn_packet::*;
pub mod rotate_head_packet;
pub use rotate_head_packet::*;
pub mod select_advancements_tab_packet;
pub use select_advancements_tab_packet::*;
pub mod set_border_packet;
pub use set_border_packet::*;
pub mod set_camera_packet;
pub use set_camera_packet::*;
pub mod set_carried_item_packet;
pub use set_carried_item_packet::*;
pub mod set_chunk_cache_center_packet;
pub use set_chunk_cache_center_packet::*;
pub mod set_chunk_cache_radius_packet;
pub use set_chunk_cache_radius_packet::*;
pub mod set_display_objective_packet;
pub use set_display_objective_packet::*;
pub mod set_entity_data_packet;
pub use set_entity_data_packet::*;
pub mod set_entity_link_packet;
pub use set_entity_link_packet::*;
pub mod set_entity_motion_packet;
pub use set_entity_motion_packet::*;
pub mod set_equipped_item_packet;
pub use set_equipped_item_packet::*;
pub mod set_experience_packet;
pub use set_experience_packet::*;
pub mod set_health_packet;
pub use set_health_packet::*;
pub mod set_objective_packet;
pub use set_objective_packet::*;
pub mod set_passengers_packet;
pub use set_passengers_packet::*;
pub mod set_player_team_packet;
pub use set_player_team_packet::*;
pub mod set_score_packet;
pub use set_score_packet::*;
pub mod set_spawn_position_packet;
pub use set_spawn_position_packet::*;
pub mod set_time_packet;
pub use set_time_packet::*;
pub mod set_titles_packet;
pub use set_titles_packet::*;
pub mod sound_entity_packet;
pub use sound_entity_packet::*;
pub mod sound_packet;
pub use sound_packet::*;
pub mod stop_sound_packet;
pub use stop_sound_packet::*;
pub mod tab_list_packet;
pub use tab_list_packet::*;
pub mod tag_query_packet;
pub use tag_query_packet::*;
pub mod take_item_entity_packet;
pub use take_item_entity_packet::*;
pub mod teleport_entity_packet;
pub use teleport_entity_packet::*;
pub mod update_advancements_packet;
pub use update_advancements_packet::*;
pub mod update_attributes_packet;
pub use update_attributes_packet::*;
pub mod update_mob_effect_packet;
pub use update_mob_effect_packet::*;
pub mod update_recipes_packet;
pub use update_recipes_packet::*;
pub mod update_tags_packet;
pub use update_tags_packet::*;

use bytes::BytesMut;
use crate::Result;
use std::io::Error as IoError;
use std::io::ErrorKind;
use crate::packet::*;
use crate::network::*;

pub fn decode_packet(id: i32, buf: &mut BytesMut) -> Result<PacketGameClientbound> {
    match id {
        0 => Ok(PacketGameClientbound::AddEntityPacket(AddEntityPacket::decode(buf)?)),
        1 => Ok(PacketGameClientbound::AddExperienceOrbPacket(AddExperienceOrbPacket::decode(buf)?)),
        2 => Ok(PacketGameClientbound::AddGlobalEntityPacket(AddGlobalEntityPacket::decode(buf)?)),
        3 => Ok(PacketGameClientbound::AddMobPacket(AddMobPacket::decode(buf)?)),
        4 => Ok(PacketGameClientbound::AddPaintingPacket(AddPaintingPacket::decode(buf)?)),
        5 => Ok(PacketGameClientbound::AddPlayerPacket(AddPlayerPacket::decode(buf)?)),
        6 => Ok(PacketGameClientbound::AnimatePacket(AnimatePacket::decode(buf)?)),
        7 => Ok(PacketGameClientbound::AwardStatsPacket(AwardStatsPacket::decode(buf)?)),
        8 => Ok(PacketGameClientbound::BlockBreakAckPacket(BlockBreakAckPacket::decode(buf)?)),
        9 => Ok(PacketGameClientbound::BlockDestructionPacket(BlockDestructionPacket::decode(buf)?)),
        10 => Ok(PacketGameClientbound::BlockEntityDataPacket(BlockEntityDataPacket::decode(buf)?)),
        11 => Ok(PacketGameClientbound::BlockEventPacket(BlockEventPacket::decode(buf)?)),
        12 => Ok(PacketGameClientbound::BlockUpdatePacket(BlockUpdatePacket::decode(buf)?)),
        13 => Ok(PacketGameClientbound::BossEventPacket(BossEventPacket::decode(buf)?)),
        14 => Ok(PacketGameClientbound::ChangeDifficultyPacket(ChangeDifficultyPacket::decode(buf)?)),
        15 => Ok(PacketGameClientbound::ChatPacket(ChatPacket::decode(buf)?)),
        16 => Ok(PacketGameClientbound::ChunkBlocksUpdatePacket(ChunkBlocksUpdatePacket::decode(buf)?)),
        17 => Ok(PacketGameClientbound::CommandSuggestionsPacket(CommandSuggestionsPacket::decode(buf)?)),
        18 => Ok(PacketGameClientbound::CommandsPacket(CommandsPacket::decode(buf)?)),
        19 => Ok(PacketGameClientbound::ContainerAckPacket(ContainerAckPacket::decode(buf)?)),
        20 => Ok(PacketGameClientbound::ContainerClosePacket(ContainerClosePacket::decode(buf)?)),
        21 => Ok(PacketGameClientbound::ContainerSetContentPacket(ContainerSetContentPacket::decode(buf)?)),
        22 => Ok(PacketGameClientbound::ContainerSetDataPacket(ContainerSetDataPacket::decode(buf)?)),
        23 => Ok(PacketGameClientbound::ContainerSetSlotPacket(ContainerSetSlotPacket::decode(buf)?)),
        24 => Ok(PacketGameClientbound::CooldownPacket(CooldownPacket::decode(buf)?)),
        25 => Ok(PacketGameClientbound::CustomPayloadPacket(CustomPayloadPacket::decode(buf)?)),
        26 => Ok(PacketGameClientbound::CustomSoundPacket(CustomSoundPacket::decode(buf)?)),
        27 => Ok(PacketGameClientbound::DisconnectPacket(DisconnectPacket::decode(buf)?)),
        28 => Ok(PacketGameClientbound::EntityEventPacket(EntityEventPacket::decode(buf)?)),
        29 => Ok(PacketGameClientbound::ExplodePacket(ExplodePacket::decode(buf)?)),
        30 => Ok(PacketGameClientbound::ForgetLevelChunkPacket(ForgetLevelChunkPacket::decode(buf)?)),
        31 => Ok(PacketGameClientbound::GameEventPacket(GameEventPacket::decode(buf)?)),
        32 => Ok(PacketGameClientbound::HorseScreenOpenPacket(HorseScreenOpenPacket::decode(buf)?)),
        33 => Ok(PacketGameClientbound::KeepAlivePacket(KeepAlivePacket::decode(buf)?)),
        34 => Ok(PacketGameClientbound::LevelChunkPacket(LevelChunkPacket::decode(buf)?)),
        35 => Ok(PacketGameClientbound::LevelEventPacket(LevelEventPacket::decode(buf)?)),
        36 => Ok(PacketGameClientbound::LevelParticlesPacket(LevelParticlesPacket::decode(buf)?)),
        37 => Ok(PacketGameClientbound::LightUpdatePacket(LightUpdatePacket::decode(buf)?)),
        38 => Ok(PacketGameClientbound::LoginPacket(LoginPacket::decode(buf)?)),
        39 => Ok(PacketGameClientbound::MapItemDataPacket(MapItemDataPacket::decode(buf)?)),
        40 => Ok(PacketGameClientbound::MerchantOffersPacket(MerchantOffersPacket::decode(buf)?)),
        41 => Err(/* TODO: NYI */)
        42 => Err(/* TODO: NYI */)
        43 => Err(/* TODO: NYI */)
        44 => Ok(PacketGameClientbound::MoveEntityPacket(MoveEntityPacket::decode(buf)?)),
        45 => Ok(PacketGameClientbound::MoveVehiclePacket(MoveVehiclePacket::decode(buf)?)),
        46 => Ok(PacketGameClientbound::OpenBookPacket(OpenBookPacket::decode(buf)?)),
        47 => Ok(PacketGameClientbound::OpenScreenPacket(OpenScreenPacket::decode(buf)?)),
        48 => Ok(PacketGameClientbound::OpenSignEditorPacket(OpenSignEditorPacket::decode(buf)?)),
        49 => Ok(PacketGameClientbound::PlaceGhostRecipePacket(PlaceGhostRecipePacket::decode(buf)?)),
        50 => Ok(PacketGameClientbound::PlayerAbilitiesPacket(PlayerAbilitiesPacket::decode(buf)?)),
        51 => Ok(PacketGameClientbound::PlayerCombatPacket(PlayerCombatPacket::decode(buf)?)),
        52 => Ok(PacketGameClientbound::PlayerInfoPacket(PlayerInfoPacket::decode(buf)?)),
        53 => Ok(PacketGameClientbound::PlayerLookAtPacket(PlayerLookAtPacket::decode(buf)?)),
        54 => Ok(PacketGameClientbound::PlayerPositionPacket(PlayerPositionPacket::decode(buf)?)),
        55 => Ok(PacketGameClientbound::RecipePacket(RecipePacket::decode(buf)?)),
        56 => Ok(PacketGameClientbound::RemoveEntitiesPacket(RemoveEntitiesPacket::decode(buf)?)),
        57 => Ok(PacketGameClientbound::RemoveMobEffectPacket(RemoveMobEffectPacket::decode(buf)?)),
        58 => Ok(PacketGameClientbound::ResourcePackPacket(ResourcePackPacket::decode(buf)?)),
        59 => Ok(PacketGameClientbound::RespawnPacket(RespawnPacket::decode(buf)?)),
        60 => Ok(PacketGameClientbound::RotateHeadPacket(RotateHeadPacket::decode(buf)?)),
        61 => Ok(PacketGameClientbound::SelectAdvancementsTabPacket(SelectAdvancementsTabPacket::decode(buf)?)),
        62 => Ok(PacketGameClientbound::SetBorderPacket(SetBorderPacket::decode(buf)?)),
        63 => Ok(PacketGameClientbound::SetCameraPacket(SetCameraPacket::decode(buf)?)),
        64 => Ok(PacketGameClientbound::SetCarriedItemPacket(SetCarriedItemPacket::decode(buf)?)),
        65 => Ok(PacketGameClientbound::SetChunkCacheCenterPacket(SetChunkCacheCenterPacket::decode(buf)?)),
        66 => Ok(PacketGameClientbound::SetChunkCacheRadiusPacket(SetChunkCacheRadiusPacket::decode(buf)?)),
        67 => Ok(PacketGameClientbound::SetDisplayObjectivePacket(SetDisplayObjectivePacket::decode(buf)?)),
        68 => Ok(PacketGameClientbound::SetEntityDataPacket(SetEntityDataPacket::decode(buf)?)),
        69 => Ok(PacketGameClientbound::SetEntityLinkPacket(SetEntityLinkPacket::decode(buf)?)),
        70 => Ok(PacketGameClientbound::SetEntityMotionPacket(SetEntityMotionPacket::decode(buf)?)),
        71 => Ok(PacketGameClientbound::SetEquippedItemPacket(SetEquippedItemPacket::decode(buf)?)),
        72 => Ok(PacketGameClientbound::SetExperiencePacket(SetExperiencePacket::decode(buf)?)),
        73 => Ok(PacketGameClientbound::SetHealthPacket(SetHealthPacket::decode(buf)?)),
        74 => Ok(PacketGameClientbound::SetObjectivePacket(SetObjectivePacket::decode(buf)?)),
        75 => Ok(PacketGameClientbound::SetPassengersPacket(SetPassengersPacket::decode(buf)?)),
        76 => Ok(PacketGameClientbound::SetPlayerTeamPacket(SetPlayerTeamPacket::decode(buf)?)),
        77 => Ok(PacketGameClientbound::SetScorePacket(SetScorePacket::decode(buf)?)),
        78 => Ok(PacketGameClientbound::SetSpawnPositionPacket(SetSpawnPositionPacket::decode(buf)?)),
        79 => Ok(PacketGameClientbound::SetTimePacket(SetTimePacket::decode(buf)?)),
        80 => Ok(PacketGameClientbound::SetTitlesPacket(SetTitlesPacket::decode(buf)?)),
        81 => Ok(PacketGameClientbound::SoundEntityPacket(SoundEntityPacket::decode(buf)?)),
        82 => Ok(PacketGameClientbound::SoundPacket(SoundPacket::decode(buf)?)),
        83 => Ok(PacketGameClientbound::StopSoundPacket(StopSoundPacket::decode(buf)?)),
        84 => Ok(PacketGameClientbound::TabListPacket(TabListPacket::decode(buf)?)),
        85 => Ok(PacketGameClientbound::TagQueryPacket(TagQueryPacket::decode(buf)?)),
        86 => Ok(PacketGameClientbound::TakeItemEntityPacket(TakeItemEntityPacket::decode(buf)?)),
        87 => Ok(PacketGameClientbound::TeleportEntityPacket(TeleportEntityPacket::decode(buf)?)),
        88 => Ok(PacketGameClientbound::UpdateAdvancementsPacket(UpdateAdvancementsPacket::decode(buf)?)),
        89 => Ok(PacketGameClientbound::UpdateAttributesPacket(UpdateAttributesPacket::decode(buf)?)),
        90 => Ok(PacketGameClientbound::UpdateMobEffectPacket(UpdateMobEffectPacket::decode(buf)?)),
        91 => Ok(PacketGameClientbound::UpdateRecipesPacket(UpdateRecipesPacket::decode(buf)?)),
        92 => Ok(PacketGameClientbound::UpdateTagsPacket(UpdateTagsPacket::decode(buf)?)),
        _ => Err(Box::new(IoError::from(ErrorKind::InvalidData)))
    }
}

pub fn encode_packet(packet: PacketGameClientbound, buf: &mut BytesMut) {
    match packet {
        PacketGameClientbound::AddEntityPacket(deref_packet) => { buf.set_mc_var_int(0); deref_packet.encode(buf); },
        PacketGameClientbound::AddExperienceOrbPacket(deref_packet) => { buf.set_mc_var_int(1); deref_packet.encode(buf); },
        PacketGameClientbound::AddGlobalEntityPacket(deref_packet) => { buf.set_mc_var_int(2); deref_packet.encode(buf); },
        PacketGameClientbound::AddMobPacket(deref_packet) => { buf.set_mc_var_int(3); deref_packet.encode(buf); },
        PacketGameClientbound::AddPaintingPacket(deref_packet) => { buf.set_mc_var_int(4); deref_packet.encode(buf); },
        PacketGameClientbound::AddPlayerPacket(deref_packet) => { buf.set_mc_var_int(5); deref_packet.encode(buf); },
        PacketGameClientbound::AnimatePacket(deref_packet) => { buf.set_mc_var_int(6); deref_packet.encode(buf); },
        PacketGameClientbound::AwardStatsPacket(deref_packet) => { buf.set_mc_var_int(7); deref_packet.encode(buf); },
        PacketGameClientbound::BlockBreakAckPacket(deref_packet) => { buf.set_mc_var_int(8); deref_packet.encode(buf); },
        PacketGameClientbound::BlockDestructionPacket(deref_packet) => { buf.set_mc_var_int(9); deref_packet.encode(buf); },
        PacketGameClientbound::BlockEntityDataPacket(deref_packet) => { buf.set_mc_var_int(10); deref_packet.encode(buf); },
        PacketGameClientbound::BlockEventPacket(deref_packet) => { buf.set_mc_var_int(11); deref_packet.encode(buf); },
        PacketGameClientbound::BlockUpdatePacket(deref_packet) => { buf.set_mc_var_int(12); deref_packet.encode(buf); },
        PacketGameClientbound::BossEventPacket(deref_packet) => { buf.set_mc_var_int(13); deref_packet.encode(buf); },
        PacketGameClientbound::ChangeDifficultyPacket(deref_packet) => { buf.set_mc_var_int(14); deref_packet.encode(buf); },
        PacketGameClientbound::ChatPacket(deref_packet) => { buf.set_mc_var_int(15); deref_packet.encode(buf); },
        PacketGameClientbound::ChunkBlocksUpdatePacket(deref_packet) => { buf.set_mc_var_int(16); deref_packet.encode(buf); },
        PacketGameClientbound::CommandSuggestionsPacket(deref_packet) => { buf.set_mc_var_int(17); deref_packet.encode(buf); },
        PacketGameClientbound::CommandsPacket(deref_packet) => { buf.set_mc_var_int(18); deref_packet.encode(buf); },
        PacketGameClientbound::ContainerAckPacket(deref_packet) => { buf.set_mc_var_int(19); deref_packet.encode(buf); },
        PacketGameClientbound::ContainerClosePacket(deref_packet) => { buf.set_mc_var_int(20); deref_packet.encode(buf); },
        PacketGameClientbound::ContainerSetContentPacket(deref_packet) => { buf.set_mc_var_int(21); deref_packet.encode(buf); },
        PacketGameClientbound::ContainerSetDataPacket(deref_packet) => { buf.set_mc_var_int(22); deref_packet.encode(buf); },
        PacketGameClientbound::ContainerSetSlotPacket(deref_packet) => { buf.set_mc_var_int(23); deref_packet.encode(buf); },
        PacketGameClientbound::CooldownPacket(deref_packet) => { buf.set_mc_var_int(24); deref_packet.encode(buf); },
        PacketGameClientbound::CustomPayloadPacket(deref_packet) => { buf.set_mc_var_int(25); deref_packet.encode(buf); },
        PacketGameClientbound::CustomSoundPacket(deref_packet) => { buf.set_mc_var_int(26); deref_packet.encode(buf); },
        PacketGameClientbound::DisconnectPacket(deref_packet) => { buf.set_mc_var_int(27); deref_packet.encode(buf); },
        PacketGameClientbound::EntityEventPacket(deref_packet) => { buf.set_mc_var_int(28); deref_packet.encode(buf); },
        PacketGameClientbound::ExplodePacket(deref_packet) => { buf.set_mc_var_int(29); deref_packet.encode(buf); },
        PacketGameClientbound::ForgetLevelChunkPacket(deref_packet) => { buf.set_mc_var_int(30); deref_packet.encode(buf); },
        PacketGameClientbound::GameEventPacket(deref_packet) => { buf.set_mc_var_int(31); deref_packet.encode(buf); },
        PacketGameClientbound::HorseScreenOpenPacket(deref_packet) => { buf.set_mc_var_int(32); deref_packet.encode(buf); },
        PacketGameClientbound::KeepAlivePacket(deref_packet) => { buf.set_mc_var_int(33); deref_packet.encode(buf); },
        PacketGameClientbound::LevelChunkPacket(deref_packet) => { buf.set_mc_var_int(34); deref_packet.encode(buf); },
        PacketGameClientbound::LevelEventPacket(deref_packet) => { buf.set_mc_var_int(35); deref_packet.encode(buf); },
        PacketGameClientbound::LevelParticlesPacket(deref_packet) => { buf.set_mc_var_int(36); deref_packet.encode(buf); },
        PacketGameClientbound::LightUpdatePacket(deref_packet) => { buf.set_mc_var_int(37); deref_packet.encode(buf); },
        PacketGameClientbound::LoginPacket(deref_packet) => { buf.set_mc_var_int(38); deref_packet.encode(buf); },
        PacketGameClientbound::MapItemDataPacket(deref_packet) => { buf.set_mc_var_int(39); deref_packet.encode(buf); },
        PacketGameClientbound::MerchantOffersPacket(deref_packet) => { buf.set_mc_var_int(40); deref_packet.encode(buf); },
        null => Err(/* TODO: NYI */)
        null => Err(/* TODO: NYI */)
        null => Err(/* TODO: NYI */)
        PacketGameClientbound::MoveEntityPacket(deref_packet) => { buf.set_mc_var_int(44); deref_packet.encode(buf); },
        PacketGameClientbound::MoveVehiclePacket(deref_packet) => { buf.set_mc_var_int(45); deref_packet.encode(buf); },
        PacketGameClientbound::OpenBookPacket(deref_packet) => { buf.set_mc_var_int(46); deref_packet.encode(buf); },
        PacketGameClientbound::OpenScreenPacket(deref_packet) => { buf.set_mc_var_int(47); deref_packet.encode(buf); },
        PacketGameClientbound::OpenSignEditorPacket(deref_packet) => { buf.set_mc_var_int(48); deref_packet.encode(buf); },
        PacketGameClientbound::PlaceGhostRecipePacket(deref_packet) => { buf.set_mc_var_int(49); deref_packet.encode(buf); },
        PacketGameClientbound::PlayerAbilitiesPacket(deref_packet) => { buf.set_mc_var_int(50); deref_packet.encode(buf); },
        PacketGameClientbound::PlayerCombatPacket(deref_packet) => { buf.set_mc_var_int(51); deref_packet.encode(buf); },
        PacketGameClientbound::PlayerInfoPacket(deref_packet) => { buf.set_mc_var_int(52); deref_packet.encode(buf); },
        PacketGameClientbound::PlayerLookAtPacket(deref_packet) => { buf.set_mc_var_int(53); deref_packet.encode(buf); },
        PacketGameClientbound::PlayerPositionPacket(deref_packet) => { buf.set_mc_var_int(54); deref_packet.encode(buf); },
        PacketGameClientbound::RecipePacket(deref_packet) => { buf.set_mc_var_int(55); deref_packet.encode(buf); },
        PacketGameClientbound::RemoveEntitiesPacket(deref_packet) => { buf.set_mc_var_int(56); deref_packet.encode(buf); },
        PacketGameClientbound::RemoveMobEffectPacket(deref_packet) => { buf.set_mc_var_int(57); deref_packet.encode(buf); },
        PacketGameClientbound::ResourcePackPacket(deref_packet) => { buf.set_mc_var_int(58); deref_packet.encode(buf); },
        PacketGameClientbound::RespawnPacket(deref_packet) => { buf.set_mc_var_int(59); deref_packet.encode(buf); },
        PacketGameClientbound::RotateHeadPacket(deref_packet) => { buf.set_mc_var_int(60); deref_packet.encode(buf); },
        PacketGameClientbound::SelectAdvancementsTabPacket(deref_packet) => { buf.set_mc_var_int(61); deref_packet.encode(buf); },
        PacketGameClientbound::SetBorderPacket(deref_packet) => { buf.set_mc_var_int(62); deref_packet.encode(buf); },
        PacketGameClientbound::SetCameraPacket(deref_packet) => { buf.set_mc_var_int(63); deref_packet.encode(buf); },
        PacketGameClientbound::SetCarriedItemPacket(deref_packet) => { buf.set_mc_var_int(64); deref_packet.encode(buf); },
        PacketGameClientbound::SetChunkCacheCenterPacket(deref_packet) => { buf.set_mc_var_int(65); deref_packet.encode(buf); },
        PacketGameClientbound::SetChunkCacheRadiusPacket(deref_packet) => { buf.set_mc_var_int(66); deref_packet.encode(buf); },
        PacketGameClientbound::SetDisplayObjectivePacket(deref_packet) => { buf.set_mc_var_int(67); deref_packet.encode(buf); },
        PacketGameClientbound::SetEntityDataPacket(deref_packet) => { buf.set_mc_var_int(68); deref_packet.encode(buf); },
        PacketGameClientbound::SetEntityLinkPacket(deref_packet) => { buf.set_mc_var_int(69); deref_packet.encode(buf); },
        PacketGameClientbound::SetEntityMotionPacket(deref_packet) => { buf.set_mc_var_int(70); deref_packet.encode(buf); },
        PacketGameClientbound::SetEquippedItemPacket(deref_packet) => { buf.set_mc_var_int(71); deref_packet.encode(buf); },
        PacketGameClientbound::SetExperiencePacket(deref_packet) => { buf.set_mc_var_int(72); deref_packet.encode(buf); },
        PacketGameClientbound::SetHealthPacket(deref_packet) => { buf.set_mc_var_int(73); deref_packet.encode(buf); },
        PacketGameClientbound::SetObjectivePacket(deref_packet) => { buf.set_mc_var_int(74); deref_packet.encode(buf); },
        PacketGameClientbound::SetPassengersPacket(deref_packet) => { buf.set_mc_var_int(75); deref_packet.encode(buf); },
        PacketGameClientbound::SetPlayerTeamPacket(deref_packet) => { buf.set_mc_var_int(76); deref_packet.encode(buf); },
        PacketGameClientbound::SetScorePacket(deref_packet) => { buf.set_mc_var_int(77); deref_packet.encode(buf); },
        PacketGameClientbound::SetSpawnPositionPacket(deref_packet) => { buf.set_mc_var_int(78); deref_packet.encode(buf); },
        PacketGameClientbound::SetTimePacket(deref_packet) => { buf.set_mc_var_int(79); deref_packet.encode(buf); },
        PacketGameClientbound::SetTitlesPacket(deref_packet) => { buf.set_mc_var_int(80); deref_packet.encode(buf); },
        PacketGameClientbound::SoundEntityPacket(deref_packet) => { buf.set_mc_var_int(81); deref_packet.encode(buf); },
        PacketGameClientbound::SoundPacket(deref_packet) => { buf.set_mc_var_int(82); deref_packet.encode(buf); },
        PacketGameClientbound::StopSoundPacket(deref_packet) => { buf.set_mc_var_int(83); deref_packet.encode(buf); },
        PacketGameClientbound::TabListPacket(deref_packet) => { buf.set_mc_var_int(84); deref_packet.encode(buf); },
        PacketGameClientbound::TagQueryPacket(deref_packet) => { buf.set_mc_var_int(85); deref_packet.encode(buf); },
        PacketGameClientbound::TakeItemEntityPacket(deref_packet) => { buf.set_mc_var_int(86); deref_packet.encode(buf); },
        PacketGameClientbound::TeleportEntityPacket(deref_packet) => { buf.set_mc_var_int(87); deref_packet.encode(buf); },
        PacketGameClientbound::UpdateAdvancementsPacket(deref_packet) => { buf.set_mc_var_int(88); deref_packet.encode(buf); },
        PacketGameClientbound::UpdateAttributesPacket(deref_packet) => { buf.set_mc_var_int(89); deref_packet.encode(buf); },
        PacketGameClientbound::UpdateMobEffectPacket(deref_packet) => { buf.set_mc_var_int(90); deref_packet.encode(buf); },
        PacketGameClientbound::UpdateRecipesPacket(deref_packet) => { buf.set_mc_var_int(91); deref_packet.encode(buf); },
        PacketGameClientbound::UpdateTagsPacket(deref_packet) => { buf.set_mc_var_int(92); deref_packet.encode(buf); },
    }
}

pub enum PacketGameClientbound {
    AddEntityPacket(AddEntityPacket),
    AddExperienceOrbPacket(AddExperienceOrbPacket),
    AddGlobalEntityPacket(AddGlobalEntityPacket),
    AddMobPacket(AddMobPacket),
    AddPaintingPacket(AddPaintingPacket),
    AddPlayerPacket(AddPlayerPacket),
    AnimatePacket(AnimatePacket),
    AwardStatsPacket(AwardStatsPacket),
    BlockBreakAckPacket(BlockBreakAckPacket),
    BlockDestructionPacket(BlockDestructionPacket),
    BlockEntityDataPacket(BlockEntityDataPacket),
    BlockEventPacket(BlockEventPacket),
    BlockUpdatePacket(BlockUpdatePacket),
    BossEventPacket(BossEventPacket),
    ChangeDifficultyPacket(ChangeDifficultyPacket),
    ChatPacket(ChatPacket),
    ChunkBlocksUpdatePacket(ChunkBlocksUpdatePacket),
    CommandSuggestionsPacket(CommandSuggestionsPacket),
    CommandsPacket(CommandsPacket),
    ContainerAckPacket(ContainerAckPacket),
    ContainerClosePacket(ContainerClosePacket),
    ContainerSetContentPacket(ContainerSetContentPacket),
    ContainerSetDataPacket(ContainerSetDataPacket),
    ContainerSetSlotPacket(ContainerSetSlotPacket),
    CooldownPacket(CooldownPacket),
    CustomPayloadPacket(CustomPayloadPacket),
    CustomSoundPacket(CustomSoundPacket),
    DisconnectPacket(DisconnectPacket),
    EntityEventPacket(EntityEventPacket),
    ExplodePacket(ExplodePacket),
    ForgetLevelChunkPacket(ForgetLevelChunkPacket),
    GameEventPacket(GameEventPacket),
    HorseScreenOpenPacket(HorseScreenOpenPacket),
    KeepAlivePacket(KeepAlivePacket),
    LevelChunkPacket(LevelChunkPacket),
    LevelEventPacket(LevelEventPacket),
    LevelParticlesPacket(LevelParticlesPacket),
    LightUpdatePacket(LightUpdatePacket),
    LoginPacket(LoginPacket),
    MapItemDataPacket(MapItemDataPacket),
    MerchantOffersPacket(MerchantOffersPacket),
    /* TODO: NYI */
    /* TODO: NYI */
    /* TODO: NYI */
    MoveEntityPacket(MoveEntityPacket),
    MoveVehiclePacket(MoveVehiclePacket),
    OpenBookPacket(OpenBookPacket),
    OpenScreenPacket(OpenScreenPacket),
    OpenSignEditorPacket(OpenSignEditorPacket),
    PlaceGhostRecipePacket(PlaceGhostRecipePacket),
    PlayerAbilitiesPacket(PlayerAbilitiesPacket),
    PlayerCombatPacket(PlayerCombatPacket),
    PlayerInfoPacket(PlayerInfoPacket),
    PlayerLookAtPacket(PlayerLookAtPacket),
    PlayerPositionPacket(PlayerPositionPacket),
    RecipePacket(RecipePacket),
    RemoveEntitiesPacket(RemoveEntitiesPacket),
    RemoveMobEffectPacket(RemoveMobEffectPacket),
    ResourcePackPacket(ResourcePackPacket),
    RespawnPacket(RespawnPacket),
    RotateHeadPacket(RotateHeadPacket),
    SelectAdvancementsTabPacket(SelectAdvancementsTabPacket),
    SetBorderPacket(SetBorderPacket),
    SetCameraPacket(SetCameraPacket),
    SetCarriedItemPacket(SetCarriedItemPacket),
    SetChunkCacheCenterPacket(SetChunkCacheCenterPacket),
    SetChunkCacheRadiusPacket(SetChunkCacheRadiusPacket),
    SetDisplayObjectivePacket(SetDisplayObjectivePacket),
    SetEntityDataPacket(SetEntityDataPacket),
    SetEntityLinkPacket(SetEntityLinkPacket),
    SetEntityMotionPacket(SetEntityMotionPacket),
    SetEquippedItemPacket(SetEquippedItemPacket),
    SetExperiencePacket(SetExperiencePacket),
    SetHealthPacket(SetHealthPacket),
    SetObjectivePacket(SetObjectivePacket),
    SetPassengersPacket(SetPassengersPacket),
    SetPlayerTeamPacket(SetPlayerTeamPacket),
    SetScorePacket(SetScorePacket),
    SetSpawnPositionPacket(SetSpawnPositionPacket),
    SetTimePacket(SetTimePacket),
    SetTitlesPacket(SetTitlesPacket),
    SoundEntityPacket(SoundEntityPacket),
    SoundPacket(SoundPacket),
    StopSoundPacket(StopSoundPacket),
    TabListPacket(TabListPacket),
    TagQueryPacket(TagQueryPacket),
    TakeItemEntityPacket(TakeItemEntityPacket),
    TeleportEntityPacket(TeleportEntityPacket),
    UpdateAdvancementsPacket(UpdateAdvancementsPacket),
    UpdateAttributesPacket(UpdateAttributesPacket),
    UpdateMobEffectPacket(UpdateMobEffectPacket),
    UpdateRecipesPacket(UpdateRecipesPacket),
    UpdateTagsPacket(UpdateTagsPacket),
}

