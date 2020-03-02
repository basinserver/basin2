pub use crate::chat::ChatComponent;
use basin2_lib::{ invalid_data, Nbt, result::* };
pub use basin2_lib::McProtoBase;
use bytes::BytesMut;
use linked_hash_map::LinkedHashMap;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::sync::{RwLock, RwLockReadGuard, RwLockWriteGuard};
use uuid::Uuid;
use std::convert::TryFrom;
pub use basin2_data::ItemStack;
use basin2_data::{ RecipeSerializer, ITEMS, ItemT };
use std::sync::atomic::Ordering;

pub fn get_var_int_len(value: i32) -> usize {
    let mut value = value as u32;
    let mut i = 1;
    while (value & !0b1111111) != 0 {
        i += 1;
        value >>= 7;
        if i > 5 {
            break;
        }
    }
    return i;
}

pub trait McProtoSpecialized: McProtoBase {
    fn get_mc_block_pos(&mut self) -> Result<BlockPos>;
    fn get_mc_item_stack(&mut self) -> Result<ItemStack>;
    fn get_mc_block_hit_result(&mut self) -> Result<BlockHitResult>;
    fn get_mc_chat_component(&mut self) -> Result<ChatComponent>;
    fn set_mc_block_pos(&mut self, value: BlockPos);
    fn set_mc_item_stack(&mut self, value: ItemStack);
    fn set_mc_block_hit_result(&mut self, value: BlockHitResult);
    fn set_mc_chat_component(&mut self, value: ChatComponent);
}

impl McProtoSpecialized for BytesMut {
    fn get_mc_block_pos(&mut self) -> Result<BlockPos> {
        let raw = self.get_mc_i64()?;
        Ok(raw.into())
    }

    fn get_mc_item_stack(&mut self) -> Result<ItemStack> {
        if self.get_mc_bool()? {
            let item_id = self.get_mc_var_int()?;
            let count = self.get_mc_i8()?;
            if count < 0 || count > 64 {
                return invalid_data();
            }
            let nbt = self.get_mc_nbt()?;
            Ok(ItemStack {
                item: ItemT::item_not_found(ITEMS.get(item_id as u32))?,
                count: count as i32,
                nbt: Some(nbt),
            })
        } else {
            Ok(ItemStack::empty())
        }
    }

    fn get_mc_block_hit_result(&mut self) -> Result<BlockHitResult> {
        let block_pos = self.get_mc_block_pos()?;
        let direction: Direction = self.get_mc_enum()?;
        let x = self.get_mc_f32()?;
        let y = self.get_mc_f32()?;
        let z = self.get_mc_f32()?;
        let inside = self.get_mc_bool()?;
        Ok(BlockHitResult {
            block_pos,
            direction,
            location: (x, y, z),
            inside,
        })
    }

    fn get_mc_chat_component(&mut self) -> Result<ChatComponent> {
        ChatComponent::parse(serde_json::from_str(&*self.get_mc_string(262144)?)?)
            .map(|value| Ok(value))
            .unwrap_or(invalid_data())
    }

    fn set_mc_block_pos(&mut self, value: BlockPos) {
        self.set_mc_i64(value.into());
    }

    fn set_mc_item_stack(&mut self, value: ItemStack) {
        if value.is_empty() {
            self.set_mc_bool(false);
            return;
        }
        self.set_mc_bool(true);
        self.set_mc_var_int(value.item.registry_id.load(Ordering::Relaxed) as i32);
        self.set_mc_i8(value.count as i8);
        // TODO: item nbt override?
        self.set_mc_nbt(value.nbt.unwrap_or(Nbt::End));
    }

    fn set_mc_block_hit_result(&mut self, value: BlockHitResult) {
        self.set_mc_block_pos(value.block_pos);
        self.set_mc_var_int(value.direction as i32);
        self.set_mc_f32(value.location.0);
        self.set_mc_f32(value.location.1);
        self.set_mc_f32(value.location.2);
        self.set_mc_bool(value.inside);
    }

    fn set_mc_chat_component(&mut self, value: ChatComponent) {
        self.set_mc_string(serde_json::to_string(&value.serialize()).unwrap());
    }
}

pub struct Command {}

pub struct BaseCommandNode {
    pub uuid: Uuid,
    pub children: LinkedHashMap<String, Arc<CommandNode>>,
    pub redirect: Option<Arc<CommandNode>>,
    pub command: bool,
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum ArgumentType {
    Bool,
    Double { min: Option<f64>, max: Option<f64> },
    Float { min: Option<f32>, max: Option<f32> },
    Integer { min: Option<i32>, max: Option<i32> },
    Long { min: Option<i64>, max: Option<i64> },
    Str(ArgumentStringType),

    Entity { single: bool, players_only: bool },
    GameProfile,
    BlockPos,
    ColumnPos,
    Vec3,
    Vec2,
    BlockState,
    BlockPredicate,
    ItemStack,
    ItemPredicate,
    Color,
    ChatComponent,
    Message,
    NbtCompoundTag,
    NbtTag,
    NbtPath,
    Objective,
    ObjectiveCriteria,
    Operation,
    Particle,
    Rotation,
    ScoreboardSlot,
    ScoreHolder { multiple: bool },
    Swizzle,
    Team,
    ItemSlot,
    ResourceLocation,
    MobEffect,
    Function,
    EntityAnchor,
    IntRange,
    FloatRange,
    ItemEnchantment,
    EntitySummon,
    Dimension,
    Time,
}

impl ArgumentType {
    pub fn name(&self) -> &'static str {
        use ArgumentType::*;
        match self {
            Bool => "brigadier:bool",
            Double { .. } => "brigadier:double",
            Float { .. } => "brigadier:float",
            Integer { .. } => "brigadier:integer",
            Long { .. } => "brigadier:long",
            Str(_) => "brigadier:string",

            Entity { .. } => "minecraft:entity",
            GameProfile => "minecraft:game_profile",
            BlockPos => "minecraft:block_pos",
            ColumnPos => "minecraft:column_pos",
            Vec3 => "minecraft:vec3",
            Vec2 => "minecraft:vec2",
            BlockState => "minecraft:block_state",
            BlockPredicate => "minecraft:block_predicate",
            ItemStack => "minecraft:item_stack",
            ItemPredicate => "minecraft:item_predicate",
            Color => "minecraft:color",
            ChatComponent => "minecraft:component",
            Message => "minecraft:message",
            NbtCompoundTag => "minecraft:nbt_compound_tag",
            NbtTag => "minecraft:nbt_tag",
            NbtPath => "minecraft:nbt_path",
            Objective => "minecraft:objective",
            ObjectiveCriteria => "minecraft:objective_criteria",
            Operation => "minecraft:operation",
            Particle => "minecraft:particle",
            Rotation => "minecraft:rotation",
            ScoreboardSlot => "minecraft:scoreboard_slot",
            ScoreHolder { .. } => "minecraft:score_holder",
            Swizzle => "minecraft:swizzle",
            Team => "minecraft:team",
            ItemSlot => "minecraft:item_slot",
            ResourceLocation => "minecraft:resource_location",
            MobEffect => "minecraft:mob_effect",
            Function => "minecraft:function",
            EntityAnchor => "minecraft:entity_anchor",
            IntRange => "minecraft:int_range",
            FloatRange => "minecraft:float_range",
            ItemEnchantment => "minecraft:item_enchantment",
            EntitySummon => "minecraft:entity_summon",
            Dimension => "minecraft:dimension",
            Time => "minecraft:time",
        }
    }
}

pub enum CommandNode {
    Root {
        node: RwLock<BaseCommandNode>,
    },
    Argument {
        node: RwLock<BaseCommandNode>,
        name: String,
        argument_type: ArgumentType,
        custom_suggestions: Option<ResourceLocation>,
    },
    Literal {
        node: RwLock<BaseCommandNode>,
        literal: String,
    },
}

impl CommandNode {
    pub fn node(&self) -> RwLockReadGuard<BaseCommandNode> {
        use CommandNode::*;

        match self {
            Root { node } | Argument { node, .. } | Literal { node, .. } => node.read().unwrap(),
        }
    }

    pub fn node_mut(&self) -> RwLockWriteGuard<BaseCommandNode> {
        use CommandNode::*;

        match self {
            Root { node } | Argument { node, .. } | Literal { node, .. } => node.write().unwrap(),
        }
    }
}

#[derive(PartialEq, Clone, Debug)]
pub struct EntityMetadataItem {
    pub id: u8,
    pub data: EntityMetadata,
}

#[derive(PartialEq, Clone, Debug)]
pub struct Recipe {
    pub id: String,
    pub serializer: RecipeSerializer,
}

#[derive(PartialEq, Clone, Debug)]
pub struct EntityAttributeModifier {
    pub uuid: Uuid,
    pub amount: f64,
    pub operation: EntityAttributeModifierOperation,
}

#[derive(PartialEq, Clone, Debug)]
pub struct EntityAttribute {
    pub name: String,
    pub base: f64,
    pub modifiers: Vec<EntityAttributeModifier>,
}

#[derive(PartialEq, Clone, Debug)]
pub struct AdvancementDisplayInfo {
    pub title: ChatComponent,
    pub description: ChatComponent,
    pub icon: ItemStack,
    pub frame: FrameType,
    pub background: Option<ResourceLocation>,
    pub showToast: bool,
    pub hidden: bool,
    pub x: f32,
    pub y: f32,
}

#[derive(PartialEq, Clone, Debug)]
pub struct Advancement {
    pub parentId: Option<ResourceLocation>,
    pub display: Option<AdvancementDisplayInfo>,
    pub criterion: Vec<String>,
    pub requirements: Vec<Vec<String>>,
}

#[derive(PartialEq, Clone, Debug)]
pub struct PlayerProperty {
    pub name: String,
    pub value: String,
    pub signature: Option<String>,
}

#[derive(PartialEq, Clone, Debug)]
pub struct Suggestion {
    pub text: String,
    pub tooltip: Option<ChatComponent>,
}

#[derive(PartialEq, Clone, Debug)]
pub struct Suggestions {
    pub start: i32,
    pub end: i32,
    pub suggestions: Vec<Suggestion>,
}

#[derive(PartialEq, Clone, Debug)]
pub struct BlockPos {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

impl Into<i64> for BlockPos {
    fn into(self) -> i64 {
        let mut raw: i64 = 0;
        raw |= (self.x as i64 & ((1 << 26) - 1)) << 38;
        raw |= self.y as i64 & ((1 << 12) - 1);
        raw |= (self.z as i64 & ((1 << 26) - 1)) << 12;
        return raw;
    }
}

impl From<i64> for BlockPos {
    fn from(raw: i64) -> BlockPos {
        BlockPos {
            x: (raw >> 38) as i32,
            y: (raw << 52 >> 52) as i32,
            z: (raw << 26 >> 38) as i32,
        }
    }
}

#[derive(PartialEq, Clone, Debug)]
pub struct BlockHitResult {
    pub location: (f32, f32, f32),
    pub direction: Direction,
    pub block_pos: BlockPos,
    pub inside: bool,
}

#[derive(PartialEq, Clone, Debug)]
pub struct ChunkPos {
    pub x: i32,
    pub z: i32,
}

#[derive(PartialEq, Clone, Debug)]
pub struct MapDecoration {
    pub decoration_type: MapDecorationType,
    pub x: u8,
    pub y: u8,
    pub rot: u8,
    pub component: Option<ChatComponent>,
}

#[derive(PartialEq, Clone, Debug)]
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

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct GameProfile {
    pub uuid: Option<Uuid>,
    pub name: String,
    pub legacy: bool,
}

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct ServerStatus {
    pub description: serde_json::Value, // ChatComponent
    pub players: ServerStatusPlayers,
    pub version: ServerStatusVersion,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub favicon: Option<String>,
}

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct ServerStatusVersion {
    pub name: String,
    pub protocol: i32,
}

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct ServerStatusPlayers {
    pub max: i32,
    pub online: i32,
    pub sample: Option<Vec<GameProfile>>,
}

#[derive(PartialEq, Clone, Debug)]
pub struct ChunkBlocksUpdatePacketBlockUpdate {
    pub offset: u16,
    pub block: BlockState,
}

pub type ResourceLocation = String;
pub type MobEffect = u8;
pub type SoundEvent = i32;
pub type BlockState = i32;
pub type Block = i32;
pub type EntityType = i32;
pub type Item = i32;

#[derive(PartialEq, Clone, Debug)]
pub enum ParticleOptions {
    Item(Particle, ItemStack),
    Dust {
        particle: Particle,
        r: f32,
        g: f32,
        b: f32,
        scale: f32,
    },
    Block(Particle, BlockState),
    Simple(Particle),
}

impl ParticleOptions {
    pub fn id(&self) -> Particle {
        use ParticleOptions::*;
        match self {
            Item(particle, ..) => *particle,
            Dust { particle, .. } => *particle,
            Block(particle, ..) => *particle,
            Simple(particle) => *particle,
        }
    }

    pub fn serialize(self, buf: &mut BytesMut) {
        use ParticleOptions::*;
        match self {
            Item(_, item) => {
                buf.set_mc_item_stack(item);
            }
            Dust { r, g, b, scale, .. } => {
                buf.set_mc_f32(r);
                buf.set_mc_f32(g);
                buf.set_mc_f32(b);
                buf.set_mc_f32(scale);
            }
            Block(_, state) => {
                buf.set_mc_var_int(state);
            }
            Simple(_) => {}
        }
    }

    pub fn parse(id: Particle, buf: &mut BytesMut) -> Result<ParticleOptions> {
        use ParticleOptions::*;
        Ok(match id {
            Particle::Item => Item(id, buf.get_mc_item_stack()?),
            Particle::Dust => Dust {
                particle: id,
                r: buf.get_mc_f32()?,
                g: buf.get_mc_f32()?,
                b: buf.get_mc_f32()?,
                scale: buf.get_mc_f32()?,
            },
            Particle::Block | Particle::FallingDust => Block(id, buf.get_mc_var_int()?),
            _ => Simple(id),
        })
    }
}

#[derive(PartialEq, Clone, Debug)]
pub enum EntityMetadata {
    Byte(i8),
    Int(i32),
    Float(f32),
    Str(String),
    Component(ChatComponent),
    OptionalComponent(Option<ChatComponent>),
    ItemStack(ItemStack),
    Boolean(bool),
    Rotations {
        x: f32,
        y: f32,
        z: f32,
    },
    BlockPos(BlockPos),
    OptionalBlockPos(Option<BlockPos>),
    Direction(Direction),
    OptionalUuid(Option<Uuid>),
    BlockState(BlockState),
    CompoundTag(Nbt),
    Particle(ParticleOptions),
    VillagerData {
        villager_type: VillagerType,
        villager_profession: VillagerProfession,
        level: i32,
    },
    OptionalUnsignedInt(Option<i32>),
    Pose(EntityPose),
}

enum_from_primitive! {
#[derive(Clone, Copy, PartialEq, Debug)]
#[repr(i32)]
pub enum Particle {
    Block,
    Bubble,
    Cloud,
    Crit,
    DamageIndicator,
    DragonBreath,
    DrippingLava,
    FallingLava,
    LandingLava,
    DrippingWater,
    FallingWater,
    Dust,
    Effect,
    ElderGuardian,
    EnchantedHit,
    Enchant,
    EndRod,
    EntityEffect,
    ExplosionEmitter,
    Explosion,
    FallingDust,
    Firework,
    Fishing,
    Flame,
    Flash,
    HappyVillager,
    Composter,
    Heart,
    InstantEffect,
    Item,
    ItemSlime,
    ItemSnowball,
    LargeSmoke,
    Lava,
    Mycelium,
    Note,
    Poof,
    Portal,
    Rain,
    Smoke,
    Sneeze,
    Spit,
    SquidInk,
    SweepAttack,
    TotemOfUndying,
    Underwater,
    Splash,
    Witch,
    BubblePop,
    CurrentDown,
    BubbleColumnUp,
    Nautilus,
    Dolphin,
    CampfireCosySmoke,
    CampfireSignalSmoke,
    DrippingHoney,
    FallingHoney,
    LandingHoney,
    FallingNectar,
}
}

enum_from_primitive! {
#[derive(Clone, Copy, PartialEq, Debug)]
#[repr(i32)]
pub enum ArgumentStringType {
    SingleWord,
    QuotablePhrase,
    GreedyPhrase
}
}

enum_from_primitive! {
#[derive(Clone, Copy, PartialEq, Debug)]
#[repr(i32)]
pub enum EntityPose {
    Standing,
    FallFlying,
    Sleeping,
    Swimming,
    SpinAttack,
    Crouching,
    Dying,
}
}

enum_from_primitive! {
#[derive(Clone, Copy, PartialEq, Debug)]
#[repr(i32)]
pub enum VillagerType {
    Desert,
    Jungle,
    Plains,
    Savanna,
    Snow,
    Swamp,
    Taiga,
}
}

enum_from_primitive! {
#[derive(Clone, Copy, PartialEq, Debug)]
#[repr(i32)]
pub enum VillagerProfession {
    None,
    Armorer,
    Butcher,
    Cartographer,
    Cleric,
    Farmer,
    Fisherman,
    Fletcher,
    Leatherworker,
    Librarian,
    Mason,
    Nitwit,
    Shepherd,
    Toolsmith,
    Weaponsmith,
}
}

enum_from_primitive! {
#[derive(Clone, Copy, PartialEq, Debug)]
#[repr(i32)]
pub enum EntityAttributeModifierOperation {
    Addition,
    MultiplyBase,
    MuliplyTotal,
}
}

enum_from_primitive! {
#[derive(Clone, Copy, PartialEq, Debug)]
#[repr(i32)]
pub enum FrameType {
    Task,
    Goal,
    Challenge,
}
}

enum_from_primitive! {
#[derive(Clone, Copy, PartialEq, Debug)]
#[repr(i32)]
pub enum DimensionType {
    Overworld,
    Nether,
    TheEnd,
}
}

impl DimensionType {
    pub fn id(&self) -> i32 {
        use DimensionType::*;
        match self {
            Overworld => 1,
            Nether => 0,
            TheEnd => 2,
        }
    }

    pub fn from_id(id: i32) -> Option<DimensionType> {
        match id {
            1 => Some(DimensionType::Overworld),
            0 => Some(DimensionType::Nether),
            2 => Some(DimensionType::TheEnd),
            _ => None,
        }
    }
}

enum_from_primitive! {
#[derive(Clone, Copy, PartialEq, Debug)]
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
#[derive(Clone, Copy, PartialEq, Debug)]
#[repr(i32)]
pub enum Difficulty {
    Peaceful,
    Easy,
    Normal,
    Hard,
}
}

impl TryFrom<String> for Difficulty {
    type Error = Error;

    fn try_from(string: String) -> Result<Difficulty> {
        Ok(match &*string {
            "peaceful" => Difficulty::Peaceful,
            "easy" => Difficulty::Easy,
            "normal" => Difficulty::Normal,
            "hard" => Difficulty::Hard,
            _ => return invalid_data(),
        })
    }
}

enum_from_primitive! {
#[derive(Clone, Copy, PartialEq, Debug)]
#[repr(i32)]
pub enum ChatVisibility {
    Full,
    System,
    Hidden,
}
}

enum_from_primitive! {
#[derive(Clone, Copy, PartialEq, Debug)]
#[repr(i32)]
pub enum HumanoidArm {
    Left,
    Right,
}
}

enum_from_primitive! {
#[derive(Clone, Copy, PartialEq, Debug)]
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
#[derive(Clone, Copy, PartialEq, Debug)]
#[repr(i32)]
pub enum InteractionHand {
    MainHand,
    OffHand,
}
}

enum_from_primitive! {
#[derive(Clone, Copy, PartialEq, Debug)]
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

impl Direction {
    pub fn get_2d_direction(&self) -> u8 {
        use Direction::*;
        match self {
            Down => 255,
            Up => 255,
            North => 2,
            South => 0,
            West => 1,
            East => 3,
        }
    }

    pub fn from_2d_direction(value: u8) -> Option<Direction> {
        match value {
            2 => Some(Direction::North),
            0 => Some(Direction::South),
            1 => Some(Direction::West),
            3 => Some(Direction::East),
            _ => None,
        }
    }
}

enum_from_primitive! {
#[derive(Clone, Copy, PartialEq, Debug)]
#[repr(i32)]
pub enum StructureMode {
    Save,
    Load,
    Corner,
    Data,
}
}

enum_from_primitive! {
#[derive(Clone, Copy, PartialEq, Debug)]
#[repr(i32)]
pub enum Mirror {
    None,
    LeftRight,
    FrontBack,
}
}

enum_from_primitive! {
#[derive(Clone, Copy, PartialEq, Debug)]
#[repr(i32)]
pub enum Rotation {
    None,
    Clockwise90,
    Closewise180,
    CounterClockwise90,
}
}

enum_from_primitive! {
#[derive(Clone, Copy, PartialEq, Debug)]
#[repr(i32)]
pub enum ChatType {
    Chat,
    System,
    GameInfo,
}
}

enum_from_primitive! {
#[derive(Clone, Copy, PartialEq, Debug)]
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
#[derive(Clone, Copy, PartialEq, Debug)]
#[repr(i32)]
pub enum GameType {
    Survival,
    Creative,
    Adventure,
    Spectator,
}
}

enum_from_primitive! {
#[derive(Clone, Copy, PartialEq, Debug)]
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
#[derive(Clone, Copy, PartialEq, Debug)]
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
#[derive(Clone, Copy, PartialEq, Debug)]
#[repr(i32)]
pub enum ConnectionProtocol {
    Handshake = -1,
    Game = 0,
    Status = 1,
    Login = 2,
}
}

enum_from_primitive! {
#[derive(Clone, Copy, PartialEq, Debug)]
#[repr(i32)]
pub enum ServerScoreboardMethod {
    Change,
    Remove,
}
}

enum_from_primitive! {
#[derive(Clone, Copy, PartialEq, Debug)]
#[repr(i32)]
pub enum StructureBlockEntityUpdateType {
    UpdateData,
    SaveArea,
    LoadArea,
    ScanArea,
}
}

enum_from_primitive! {
#[derive(Clone, Copy, PartialEq, Debug)]
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
#[derive(Clone, Copy, PartialEq, Debug)]
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
#[derive(Clone, Copy, PartialEq, Debug)]
#[repr(i32)]
pub enum EntityAnchor {
    Feet,
    Eyes,
}
}

enum_from_primitive! {
#[derive(Clone, Copy, PartialEq, Debug)]
#[repr(i32)]
pub enum ObjectiveCriteriaRenderType {
    Integer,
    Hearts,
}
}

enum_from_primitive! {
#[derive(Clone, Copy, PartialEq, Debug)]
#[repr(i32)]
pub enum CommandBlockEntityMode {
    Sequence,
    Auto,
    Redstone,
}
}

enum_from_primitive! {
#[derive(Clone, Copy, PartialEq, Debug)]
#[repr(i32)]
pub enum ClientCommandPacketAction {
    PerformRespawn,
    RequestStats,
}
}

enum_from_primitive! {
#[derive(Clone, Copy, PartialEq, Debug)]
#[repr(i32)]
pub enum InteractPacketAction {
    Interact,
    Attack,
    InteractAt,
}
}

enum_from_primitive! {
#[derive(Clone, Copy, PartialEq, Debug)]
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
#[derive(Clone, Copy, PartialEq, Debug)]
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
#[derive(Clone, Copy, PartialEq, Debug)]
#[repr(i32)]
pub enum RecipeBookUpdatePacketPurpose {
    Shown,
    Settings,
}
}

enum_from_primitive! {
#[derive(Clone, Copy, PartialEq, Debug)]
#[repr(i32)]
pub enum ResourcePackPacketAction {
    SuccessfullyLoaded,
    Declined,
    FailedDownload,
    Accepted,
}
}

enum_from_primitive! {
#[derive(Clone, Copy, PartialEq, Debug)]
#[repr(i32)]
pub enum SeenAdvancementsPacketAction {
    OpenedTab,
    ClosedScreen,
}
}

enum_from_primitive! {
#[derive(Clone, Copy, PartialEq, Debug)]
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
#[derive(Clone, Copy, PartialEq, Debug)]
#[repr(i32)]
pub enum PlayerCombatPacketEvent {
    EnterCombat,
    EndCombat,
    EntityDied,
}
}

enum_from_primitive! {
#[derive(Clone, Copy, PartialEq, Debug)]
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
#[derive(Clone, Copy, PartialEq, Debug)]
#[repr(i32)]
pub enum RecipePacketState {
    Init,
    Add,
    Remove,
}
}

enum_from_primitive! {
#[derive(Clone, Copy, PartialEq, Debug)]
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
#[derive(Clone, Copy, PartialEq, Debug)]
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
