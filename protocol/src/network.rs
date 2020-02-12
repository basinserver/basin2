
use bytes::BytesMut;
use bytes::buf::BufMut;
use uuid::Uuid;
use crate::nbt::Nbt;
use crate::Result;
use serde::{Deserialize, Serialize};

pub trait McNetwork {
    fn read_var_int(&self) -> Option<(i32, usize)>;
    fn write_var_int(&mut self, value: i32) -> usize;
}

impl McNetwork for BytesMut {
    // these are for network-layer operations and not recommended for packet-layer operations (see set_var_int and get_var_int)

    fn read_var_int(&self) -> Option<(i32, usize)> {
        let mut output: i32 = 0;
        let mut i = 0;
        let mut current_byte: u8 = 0xff;
        while (current_byte & 128) == 128 {
            if i >= self.len() {
                return None;
            }
            current_byte = self[i];
            output |= ((current_byte as u32 & 127) << (i * 7)) as i32;
            i += 1;
            if i > 5 {
                break;
            }
        }
        Some((output, i))
    }

    fn write_var_int(&mut self, mut value: i32) -> usize {
        let mut i = 0;
        while (value & -128) != 0 {
            self.put_u8((value as u8 & 127) | 128);
            value >>= 7;
            i += 1;
        }
        self.put_u8(value as u8);
        i + 1
    }

}

pub fn get_var_int_len(value: i32) -> usize {
    for i in 1..10 {
        if ((value & -1) << (i * 7)) == 0 {
            return i;
        }
    }
    return 10;
}

pub trait McPacketBuf {
    fn get_mc_var_int(&mut self) -> Result<i32>;
    fn get_mc_block_pos(&mut self) -> Result<BlockPos>;
    fn get_mc_string(&mut self) -> Result<String>;
    fn get_mc_string_bounded(&mut self, bound: i32) -> Result<String>;
    fn get_mc_u8(&mut self) -> Result<u8>;
    fn get_mc_bool(&mut self) -> Result<bool>;
    fn get_mc_i16(&mut self) -> Result<i16>;
    fn get_mc_item_stack(&mut self) -> Result<ItemStack>;
    fn get_mc_i64(&mut self) -> Result<i64>;
    fn get_mc_f64(&mut self) -> Result<f64>;
    fn get_mc_f32(&mut self) -> Result<f32>;
    fn get_mc_uuid(&mut self) -> Result<Uuid>;
    fn get_mc_block_hit_result(&mut self) -> Result<BlockHitResult>;
    fn get_mc_i32(&mut self) -> Result<i32>;
    fn get_mc_nbt(&mut self) -> Result<Nbt>;
    fn get_mc_chat_component(&mut self) -> Result<ChatComponent>;
    fn get_mc_var_int_array(&mut self) -> Result<Vec<i32>>;
    fn get_mc_u16(&mut self) -> Result<u16>;
    fn get_mc_byte_array(&mut self) -> Result<Vec<u8>>;
    fn set_mc_var_int(&mut self, value: i32);
    fn set_mc_block_pos(&mut self, value: BlockPos);
    fn set_mc_string(&mut self, value: String);
    fn set_mc_u8(&mut self, value: u8);
    fn set_mc_bool(&mut self, value: bool);
    fn set_mc_i16(&mut self, value: i16);
    fn set_mc_item_stack(&mut self, value: ItemStack);
    fn set_mc_i64(&mut self, value: i64);
    fn set_mc_f64(&mut self, value: f64);
    fn set_mc_f32(&mut self, value: f32);
    fn set_mc_uuid(&mut self, value: &Uuid);
    fn set_mc_block_hit_result(&mut self, value: BlockHitResult);
    fn set_mc_i32(&mut self, value: i32);
    fn set_mc_nbt(&mut self, value: &Nbt);
    fn set_mc_chat_component(&mut self, value: ChatComponent);
    fn set_mc_var_int_array(&mut self, value: Vec<i32>);
    fn set_mc_u16(&mut self, value: u16);
    fn set_mc_byte_array(&mut self, value: Vec<u8>);
}

impl McPacketBuf for BytesMut {

}

pub struct BlockPos {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

pub struct BlockHitResult {
    pub miss: bool,
    pub location: (f64, f64, f64),
    pub direction: Direction,
    pub blockPos: BlockPos,
    pub inside: bool,
}

pub struct ItemStack {
    pub count: i32,
    pub item_id: i32,
    pub nbt: Option<Nbt>,
}

pub struct ChunkPos {
    pub x: i32,
    pub y: i32,
}

pub struct MapDecoration {
    pub decoration_type: MapDecorationType,
    pub x: u8,
    pub y: u8,
    pub rot: u8,
    pub component: ChatComponent,
}

pub struct MerchantOffer {
    pub base_cost_a: ItemStack,
    pub cost_b: ItemStack,
    pub result: ItemStack,
    pub uses: i32,
    pub maxUses: i32,
    pub rewardExp: bool,
    pub specialPriceDiff: i32,
    pub demand: i32,
    pub priceMultiplier: f32,
    pub xp: i32,
}

#[derive(Serialize, Deserialize)]
pub struct GameProfile {
    pub uuid: Uuid,
    pub name: String,
    pub legacy: bool,
}

#[derive(Serialize, Deserialize)]
pub struct ServerStatus {
    pub description: ChatComponent,
    pub players: ServerStatusPlayers,
    pub version: ServerStatusVersion,
    pub favicon: String,
}

#[derive(Serialize, Deserialize)]
pub struct ServerStatusVersion {
    pub name: String,
    pub version: i32,
}

#[derive(Serialize, Deserialize)]
pub struct ServerStatusPlayers {
    pub max: i32,
    pub online: i32,
    pub sample: Option<Vec<GameProfile>>,
}

pub struct ChunkBlocksUpdatePacketBlockUpdate {
    pub offset: u16,
    pub block: BlockState,
}

type ResourceLocation = String;
type ChatComponent = String; // TODO this should be a struct
type MobEffect = u8;
type SoundEvent = i32;
type BlockState = i32;
type EntityType = i32;

enum_from_primitive! {
#[repr(i32)]
pub enum MapDecorationType {
    Player,
    Frame,
    RedMarker,
    BlueMarker,
    TargetX,
    TargetPoint,
    PlayerOffMap,
    PlayerOffLimits,
    Mansion,
    Monument,
    BannerWhite,
    BannerOrange,
    BannerMagenta,
    BannerLightBlue,
    BannerYellow,
    BannerLime,
    BannerPink,
    BannerGray,
    BannerLightGray,
    BannerCyan,
    BannerPurple,
    BannerBlue,
    BannerBrown,
    BannerGreen,
    BannerRed,
    BannerBlack,
    RedX,
}
}

enum_from_primitive! {
#[repr(i32)]
pub enum Difficulty {
    Peaceful,
    Easy,
    Normal,
    Hard,
}
}

enum_from_primitive! {
#[repr(i32)]
pub enum ChatVisibility {
    Full,
    System,
    Hidden,
}
}

enum_from_primitive! {
#[repr(i32)]
pub enum HumanoidArm {
    Left,
    Right,
}
}

enum_from_primitive! {
#[repr(i32)]
pub enum ClickType {
    Pickup,
    QuickMove,
    Swap,
    ClickClone,
    Throw,
    QuickCraft,
    PickupAll,
}
}

enum_from_primitive! {
#[repr(i32)]
pub enum InteractionHand {
    MainHand,
    OffHand,
}
}

enum_from_primitive! {
#[repr(i32)]
pub enum Direction {
    Down,
    Up,
    North,
    South,
    West,
    East,
}
}

enum_from_primitive! {
#[repr(i32)]
pub enum StructureMode {
    Save,
    Load,
    Corner,
    Data,
}
}

enum_from_primitive! {
#[repr(i32)]
pub enum Mirror {
    None,
    LeftRight,
    FrontBack,
}
}

enum_from_primitive! {
#[repr(i32)]
pub enum Rotation {
    None,
    Clockwise90,
    Closewise180,
    CounterClockwise90,
}
}

enum_from_primitive! {
#[repr(i32)]
pub enum ChatType {
    Chat,
    System,
    GameInfo,
}
}

enum_from_primitive! {
#[repr(i32)]
pub enum SoundSource {
    Master,
    Music,
    Records,
    Weather,
    Blocks,
    Hostile,
    Neutral,
    Players,
    Ambient,
    Voice,
}
}

enum_from_primitive! {
#[repr(i32)]
pub enum GameType {
    Survival,
    Creative,
    Adventure,
    Spectator,
}
}

enum_from_primitive! {
#[repr(i32)]
pub enum EquipmentSlot {
    MainHand,
    OffHand,
    Feet,
    Legs,
    Chest,
    Head,
}
}

enum_from_primitive! {
#[repr(i32)]
pub enum ChatFormatting {
    Black,
    DarkBlue,
    DarkGreen,
    DarkAqua,
    DarkRed,
    DarkPurple,
    Gold,
    Gray,
    DarkGray,
    Blue,
    Green,
    Aqua,
    Red,
    LightPurple,
    Yellow,
    White,
    Obfuscated,
    Bold,
    Strikethrough,
    Underline,
    Italic,
    Reset,
}
}

enum_from_primitive! {
#[repr(i32)]
pub enum ConnectionProtocol {
    Handshake,
    Game,
    Status,
    Login,
}
}

enum_from_primitive! {
#[repr(i32)]
pub enum ServerScoreboardMethod {
    Change,
    Remove,
}
}

enum_from_primitive! {
#[repr(i32)]
pub enum StructureBlockEntityUpdateType {
    UpdateData,
    SaveArea,
    LoadArea,
    ScanArea,
}
}

enum_from_primitive! {
#[repr(i32)]
pub enum BossBarColor {
    Pink,
    Blue,
    Red,
    Green,
    Yellow,
    Purple,
    White,
}
}

enum_from_primitive! {
#[repr(i32)]
pub enum BossBarOverlay {
    Progress,
    Notched6,
    Notched10,
    Notched12,
    Notched20,
}
}

enum_from_primitive! {
#[repr(i32)]
pub enum EntityAnchor {
    Feet,
    Eyes,
}
}

enum_from_primitive! {
#[repr(i32)]
pub enum ObjectiveCriteriaRenderType {
    Integer,
    Hearts,
}
}

enum_from_primitive! {
#[repr(i32)]
pub enum CommandBlockEntityMode {
    Sequence,
    Auto,
    Redstone,
}
}

enum_from_primitive! {
#[repr(i32)]
pub enum ClientCommandPacketAction {
    PerformRespawn,
    RequestStats,
}
}

enum_from_primitive! {
#[repr(i32)]
pub enum InteractPacketAction {
    Interact,
    Attack,
    InteractAt,
}
}

enum_from_primitive! {
#[repr(i32)]
pub enum PlayerActionPacketAction {
    StartDestroyBlock,
    AbortDestroyBlock,
    StopDestroyBlock,
    DropAllItems,
    DropItem,
    ReleaseUseItem,
    SwapHeldItems,
}
}

enum_from_primitive! {
#[repr(i32)]
pub enum PlayerCommandPacketAction {
    PressShiftKey,
    ReleaseShiftKey,
    StopSleeping,
    StartSprinting,
    StopSprinting,
    StartRidingJump,
    StopRidingJump,
    OpenInventory,
    StartFallFlying,
}
}

enum_from_primitive! {
#[repr(i32)]
pub enum RecipeBookUpdatePacketPurpose {
    Shown,
    Settings,
}
}

enum_from_primitive! {
#[repr(i32)]
pub enum ResourcePackPacketAction {
    SuccessfullyLoaded,
    Declined,
    FailedDownload,
    Accepted,
}
}

enum_from_primitive! {
#[repr(i32)]
pub enum SeenAdvancementsPacketAction {
    OpenedTab,
    ClosedScreen,
}
}

enum_from_primitive! {
#[repr(i32)]
pub enum BossEventPacketOperation {
    Add,
    Remove,
    UpdatePct,
    UpdateName,
    UpdateStyle,
    UpdateProperties,
}
}

enum_from_primitive! {
#[repr(i32)]
pub enum PlayerCombatPacketEvent {
    EnterCombat,
    EndCombat,
    EntityDied,
}
}

enum_from_primitive! {
#[repr(i32)]
pub enum PlayerInfoPacketAction {
    AddPlayer,
    UpdateGameMode,
    UpdateLatency,
    UpdateDisplayName,
    RemovePlayer,
}
}

enum_from_primitive! {
#[repr(i32)]
pub enum RecipePacketState {
    Init,
    Add,
    Remove,
}
}

enum_from_primitive! {
#[repr(i32)]
pub enum SetBorderPacketType {
    SetSize,
    LerpSize,
    SetCenter,
    Initialize,
    SetWarningTime,
    SetWarningBlocks,
}
}

enum_from_primitive! {
#[repr(i32)]
pub enum SetTitlesPacketType {
    Title,
    Subtitle,
    Actionbar,
    Times,
    Clear,
    Reset,
}
}
/*
complex:
Suggestions
RootCommandNode
CommandNode
ParticleOptions
TagManager

EntityType
BlockState
ClientboundChunkBlocksUpdatePacket.BlockUpdate[]
ServerStatus
PublicKey
GameProfile
SoundEvent
MobEffect
MerchantOffers
MapDecoration[]
LevelType
DimensionType
ChunkPos
ChunkBiomeContainer
*/

/*
enums: [done]

Difficulty
ChatVisiblity
HumanoidArm
ClickType
InteractionHand
Direction
StructureMode
Mirror
Rotation
ChatType
SoundSource
GameType
EquipmentSlot
ChatFormatting
ConnectionProtocol
*/

/*
internal scoped enums: [done]

CommandBlockEntity.Mode
StructureBlockEntity.UpdateType
BossEvent.BossBarColor
BossEvent.BossBarOverlay
EntityAnchorArgument.Anchor
ObjectiveCriteria.RenderType
ServerScoreboard.Method
*/

/*
packet scoped enums: [done]
ServerboundClientCommandPacket.Action
ServerboundInteractPacket.Action
ServerboundPlayerActionPacket.Action
ServerboundPlayerCommandPacket.Action
ServerboundRecipeBookUpdatePacket.Purpose
ServerboundResourcePackPacket.Action
ServerboundSeenAdvancementsPacket.Action
ClientboundBossEventPacket.Operation
ClientboundPlayerCombatPacket.Event
ClientboundPlayerInfoPacket.Action
ClientboundRecipePacket.State
ClientboundSetBorderPacket.Type
ClientboundSetTitlesPacket.Type
*/