use crate::nbt::Nbt;
use crate::result::*;
use bytes::buf::Buf;
use bytes::buf::BufMut;
use bytes::BytesMut;
use enum_primitive::FromPrimitive;
use linked_hash_map::LinkedHashMap;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::sync::{RwLock, RwLockReadGuard, RwLockWriteGuard};
use uuid::Uuid;

pub trait McNetwork {
    fn read_var_int(&self) -> Option<(i32, usize)>;
    fn write_var_int(&mut self, value: i32) -> usize;
    fn read_var_long(&self) -> Option<(i64, usize)>;
    fn write_var_long(&mut self, value: i64) -> usize;
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
        self.reserve(6);
        let mut i = 0;
        while (value & -128) != 0 {
            self.put_u8((value as u8 & 127) | 128);
            value >>= 7;
            i += 1;
        }
        self.put_u8(value as u8);
        i + 1
    }

    fn read_var_long(&self) -> Option<(i64, usize)> {
        let mut output: i64 = 0;
        let mut i = 0;
        let mut current_byte: u8 = 0xff;
        while (current_byte & 128) == 128 {
            if i >= self.len() {
                return None;
            }
            current_byte = self[i];
            output |= ((current_byte as u64 & 127) << (i * 7)) as i64;
            i += 1;
            if i > 10 {
                break;
            }
        }
        Some((output, i))
    }

    fn write_var_long(&mut self, mut value: i64) -> usize {
        self.reserve(12);
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
    fn get_mc_var_long(&mut self) -> Result<i64>;
    fn get_mc_block_pos(&mut self) -> Result<BlockPos>;
    fn get_mc_string(&mut self, bound: i32) -> Result<String>;
    fn get_mc_u8(&mut self) -> Result<u8>;
    fn get_mc_i8(&mut self) -> Result<i8>;
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
    fn get_mc_byte_array_bounded(&mut self, bound: i32) -> Result<Vec<u8>>;
    fn get_mc_enum<T: FromPrimitive>(&mut self) -> Result<T>;
    fn get_mc_enum_i32<T: FromPrimitive>(&mut self) -> Result<T>;
    fn get_mc_enum_u8<T: FromPrimitive>(&mut self) -> Result<T>;
    fn set_mc_var_int(&mut self, value: i32);
    fn set_mc_var_long(&mut self, value: i64);
    fn set_mc_block_pos(&mut self, value: BlockPos);
    fn set_mc_string(&mut self, value: String);
    fn set_mc_u8(&mut self, value: u8);
    fn set_mc_i8(&mut self, value: i8);
    fn set_mc_bool(&mut self, value: bool);
    fn set_mc_i16(&mut self, value: i16);
    fn set_mc_item_stack(&mut self, value: ItemStack);
    fn set_mc_i64(&mut self, value: i64);
    fn set_mc_f64(&mut self, value: f64);
    fn set_mc_f32(&mut self, value: f32);
    fn set_mc_uuid(&mut self, value: Uuid);
    fn set_mc_block_hit_result(&mut self, value: BlockHitResult);
    fn set_mc_i32(&mut self, value: i32);
    fn set_mc_nbt(&mut self, value: Nbt);
    fn set_mc_chat_component(&mut self, value: ChatComponent);
    fn set_mc_var_int_array(&mut self, value: Vec<i32>);
    fn set_mc_u16(&mut self, value: u16);
    fn set_mc_byte_array(&mut self, value: Vec<u8>);

    fn clone_bounded(&mut self, bound: i32) -> Result<Self>
    where
        Self: Sized;
}

pub fn invalidData<T>() -> Result<T> {
    return Err(Box::new(IoError::from(ErrorKind::InvalidData)));
}

impl McPacketBuf for BytesMut {
    fn get_mc_var_int(&mut self) -> Result<i32> {
        match self.read_var_int() {
            Some((output, length)) => {
                self.advance(length);
                Ok(output)
            }
            None => invalidData(),
        }
    }

    fn get_mc_var_long(&mut self) -> Result<i64> {
        match self.read_var_long() {
            Some((output, length)) => {
                self.advance(length);
                Ok(output)
            }
            None => invalidData(),
        }
    }

    fn get_mc_block_pos(&mut self) -> Result<BlockPos> {
        let raw = self.get_mc_i64()?;
        Ok(BlockPos {
            x: (raw >> 38) as i32,
            y: (raw << 52 >> 52) as i32,
            z: (raw << 26 >> 38) as i32,
        })
    }

    fn get_mc_string(&mut self, bound: i32) -> Result<String> {
        let length = self.get_mc_var_int()?;
        if length > bound * 4 || length < 0 || length as usize > self.len() {
            invalidData()
        } else {
            let string_value = String::from_utf8(self.split_to(length as usize).to_vec())?;
            if string_value.len() > bound as usize {
                invalidData()
            } else {
                Ok(string_value)
            }
        }
    }

    fn get_mc_u8(&mut self) -> Result<u8> {
        if self.len() < 1 {
            invalidData()
        } else {
            Ok(self.get_u8())
        }
    }

    fn get_mc_i8(&mut self) -> Result<i8> {
        if self.len() < 1 {
            invalidData()
        } else {
            Ok(self.get_i8())
        }
    }

    fn get_mc_bool(&mut self) -> Result<bool> {
        if self.len() < 1 {
            invalidData()
        } else {
            Ok(self.get_u8() != 0)
        }
    }

    fn get_mc_i16(&mut self) -> Result<i16> {
        if self.len() < 2 {
            invalidData()
        } else {
            Ok(self.get_i16())
        }
    }

    fn get_mc_item_stack(&mut self) -> Result<ItemStack> {
        if self.get_mc_bool()? {
            let item_id = self.get_mc_var_int()?;
            let count = self.get_mc_i8()?;
            if count < 0 || count > 64 {
                return invalidData();
            }
            let nbt = self.get_mc_nbt()?;
            Ok(ItemStack {
                item_id,
                count: count as i32,
                nbt: Some(nbt),
            })
        } else {
            Ok(ItemStack::empty())
        }
    }

    fn get_mc_i64(&mut self) -> Result<i64> {
        if self.len() < 8 {
            invalidData()
        } else {
            Ok(self.get_i64())
        }
    }

    fn get_mc_f64(&mut self) -> Result<f64> {
        if self.len() < 8 {
            invalidData()
        } else {
            Ok(self.get_f64())
        }
    }

    fn get_mc_f32(&mut self) -> Result<f32> {
        if self.len() < 4 {
            invalidData()
        } else {
            Ok(self.get_f32())
        }
    }

    fn get_mc_uuid(&mut self) -> Result<Uuid> {
        if self.len() < 16 {
            invalidData()
        } else {
            Ok(Uuid::from_bytes(&self.split_to(16).to_vec())?)
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

    fn get_mc_i32(&mut self) -> Result<i32> {
        if self.len() < 4 {
            invalidData()
        } else {
            Ok(self.get_i32())
        }
    }

    fn get_mc_nbt(&mut self) -> Result<Nbt> {
        Ok(Nbt::parse(self)?)
    }

    fn get_mc_chat_component(&mut self) -> Result<ChatComponent> {
        self.get_mc_string(262144)
    }

    fn get_mc_var_int_array(&mut self) -> Result<Vec<i32>> {
        let mut out: Vec<i32> = vec![];
        while let Ok(i) = self.get_mc_var_int() {
            out.push(i);
        }
        Ok(out)
    }

    fn get_mc_u16(&mut self) -> Result<u16> {
        if self.len() < 2 {
            invalidData()
        } else {
            Ok(self.get_u16())
        }
    }

    fn get_mc_byte_array(&mut self) -> Result<Vec<u8>> {
        Ok(self.to_vec())
    }

    fn get_mc_byte_array_bounded(&mut self, bound: i32) -> Result<Vec<u8>> {
        let length = self.get_mc_var_int()?;
        if length > bound || length < 0 || length as usize > self.len() {
            return invalidData();
        }
        Ok(self.split_to(length as usize).to_vec())
    }

    fn get_mc_enum<T: FromPrimitive>(&mut self) -> Result<T> {
        let action = T::from_i32(self.get_mc_var_int()?);
        let action = match action {
            Some(action) => action,
            None => {
                return Err(Box::new(IoError::from(ErrorKind::InvalidData)));
            }
        };
        return Ok(action);
    }

    fn get_mc_enum_i32<T: FromPrimitive>(&mut self) -> Result<T> {
        let action = T::from_i32(self.get_mc_i32()?);
        let action = match action {
            Some(action) => action,
            None => {
                return Err(Box::new(IoError::from(ErrorKind::InvalidData)));
            }
        };
        return Ok(action);
    }

    fn get_mc_enum_u8<T: FromPrimitive>(&mut self) -> Result<T> {
        let action = T::from_u8(self.get_mc_u8()?);
        let action = match action {
            Some(action) => action,
            None => {
                return Err(Box::new(IoError::from(ErrorKind::InvalidData)));
            }
        };
        return Ok(action);
    }

    fn set_mc_var_int(&mut self, value: i32) {
        self.write_var_int(value);
    }

    fn set_mc_var_long(&mut self, value: i64) {
        self.write_var_long(value);
    }

    fn set_mc_block_pos(&mut self, value: BlockPos) {
        let mut raw: i64 = 0;
        raw |= (value.x as i64 & ((1 << 26) - 1)) << 38;
        raw |= value.y as i64 & ((1 << 12) - 1);
        raw |= (value.z as i64 & ((1 << 26) - 1)) << 12;
        self.set_mc_i64(raw);
    }

    fn set_mc_string(&mut self, value: String) {
        let bytes = value.as_bytes();
        self.set_mc_var_int(bytes.len() as i32);
        self.extend(bytes);
    }

    fn set_mc_u8(&mut self, value: u8) {
        self.reserve(1);
        self.put_u8(value);
    }

    fn set_mc_i8(&mut self, value: i8) {
        self.reserve(1);
        self.put_i8(value);
    }

    fn set_mc_bool(&mut self, value: bool) {
        self.reserve(1);
        self.put_u8(if value { 1 } else { 0 });
    }

    fn set_mc_i16(&mut self, value: i16) {
        self.reserve(2);
        self.put_i16(value);
    }

    fn set_mc_item_stack(&mut self, value: ItemStack) {
        if value.is_empty() {
            self.set_mc_bool(false);
            return;
        }
        self.set_mc_bool(true);
        self.set_mc_var_int(value.item_id);
        self.set_mc_i8(value.count as i8);
        // TODO: item nbt override?
        self.set_mc_nbt(value.nbt.unwrap_or(Nbt::End));
    }

    fn set_mc_i64(&mut self, value: i64) {
        self.reserve(8);
        self.put_i64(value);
    }

    fn set_mc_f64(&mut self, value: f64) {
        self.reserve(8);
        self.put_f64(value);
    }

    fn set_mc_f32(&mut self, value: f32) {
        self.reserve(4);
        self.put_f32(value);
    }

    fn set_mc_uuid(&mut self, value: Uuid) {
        self.reserve(16);
        self.extend_from_slice(value.as_bytes());
    }

    fn set_mc_block_hit_result(&mut self, value: BlockHitResult) {
        self.set_mc_block_pos(value.block_pos);
        self.set_mc_var_int(value.direction as i32);
        self.set_mc_f32(value.location.0);
        self.set_mc_f32(value.location.1);
        self.set_mc_f32(value.location.2);
        self.set_mc_bool(value.inside);
    }

    fn set_mc_i32(&mut self, value: i32) {
        self.reserve(4);
        self.put_i32(value);
    }

    fn set_mc_nbt(&mut self, value: Nbt) {
        value.serialize(self);
    }

    fn set_mc_chat_component(&mut self, value: ChatComponent) {
        self.set_mc_string(value);
    }

    fn set_mc_var_int_array(&mut self, value: Vec<i32>) {
        self.set_mc_var_int(value.len() as i32);
        for i in value {
            self.set_mc_var_int(i);
        }
    }

    fn set_mc_u16(&mut self, value: u16) {
        self.reserve(2);
        self.put_u16(value);
    }

    fn set_mc_byte_array(&mut self, value: Vec<u8>) {
        self.set_mc_var_int(value.len() as i32);
        self.extend(value);
    }

    fn clone_bounded(&mut self, bound: i32) -> Result<BytesMut> {
        if self.len() > bound as usize {
            return invalidData();
        }
        let returned = self.clone();
        let advanced = self.len();
        self.advance(advanced);
        Ok(returned)
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
pub struct SimpleCookingSerializer {
    pub group: String,
    pub ingredient: Vec<ItemStack>,
    pub result: ItemStack,
    pub experience: f32,
    pub cookingTime: i32,
}

#[derive(PartialEq, Clone, Debug)]
pub enum RecipeSerializer {
    CraftingShaped {
        width: i32,
        height: i32,
        group: String,
        recipeItems: Vec<Vec<ItemStack>>,
        result: ItemStack,
    },
    CraftingShapeless {
        group: String,
        ingredients: Vec<Vec<ItemStack>>,
        result: ItemStack,
    },
    CraftingSpecialArmordye,
    CraftingSpecialBookcloning,
    CraftingSpecialMapcloning,
    CraftingSpecialMapextending,
    CraftingSpecialFireworkRocket,
    CraftingSpecialFireworkStar,
    CraftingSpecialFireworkStarFade,
    CraftingSpecialTippedarrow,
    CraftingSpecialBannerduplicate,
    CraftingSpecialShielddecoration,
    CraftingSpecialShulkerboxcoloring,
    CraftingSpecialSuspiciousstew,
    CraftingSpecialRepairitem,
    Smelting(SimpleCookingSerializer),
    Blasting(SimpleCookingSerializer),
    Smoking(SimpleCookingSerializer),
    CampfireCooking(SimpleCookingSerializer),
    Stonecutting {
        group: String,
        ingredient: Vec<ItemStack>,
        result: ItemStack,
    },
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

#[derive(PartialEq, Clone, Debug)]
pub struct BlockHitResult {
    pub location: (f32, f32, f32),
    pub direction: Direction,
    pub block_pos: BlockPos,
    pub inside: bool,
}

#[derive(PartialEq, Clone, Debug)]
pub struct ItemStack {
    pub count: i32,
    pub item_id: i32,
    pub nbt: Option<Nbt>,
}

impl ItemStack {
    pub fn is_empty(&self) -> bool {
        return self.count <= 0 || self.item_id <= 0;
    }

    pub fn empty() -> ItemStack {
        ItemStack {
            item_id: 0,
            count: 0,
            nbt: None,
        }
    }
}

#[derive(PartialEq, Clone, Debug)]
pub struct ChunkPos {
    pub x: i32,
    pub y: i32,
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
    pub description: ChatComponent,
    pub players: ServerStatusPlayers,
    pub version: ServerStatusVersion,
    pub favicon: Option<String>,
}

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct ServerStatusVersion {
    pub name: String,
    pub version: i32,
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
pub type ChatComponent = String; // TODO this should be a struct
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
            2 => Some(Direction::Down),
            0 => Some(Direction::Up),
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
    Handshake,
    Game,
    Status,
    Login,
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
