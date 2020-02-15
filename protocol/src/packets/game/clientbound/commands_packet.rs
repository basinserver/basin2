use crate::network::*;
use crate::packet::*;
use crate::result::*;
use bytes::BytesMut;
use either::Either;
use linked_hash_map::LinkedHashMap;
use std::collections::{HashMap, VecDeque};
use std::sync::Arc;
use std::sync::RwLock;
use uuid::Uuid;

impl ArgumentType {
    pub fn serialize(self, buf: &mut BytesMut) {
        use ArgumentType::*;
        match self {
            Double { min, max } => {
                let mut flags = 0;
                if min.is_some() {
                    flags |= 1;
                }
                if max.is_some() {
                    flags |= 2;
                }
                buf.set_mc_u8(flags);
                match min {
                    Some(min) => buf.set_mc_f64(min),
                    _ => (),
                }
                match max {
                    Some(max) => buf.set_mc_f64(max),
                    _ => (),
                }
            }
            Float { min, max } => {
                let mut flags = 0;
                if min.is_some() {
                    flags |= 1;
                }
                if max.is_some() {
                    flags |= 2;
                }
                buf.set_mc_u8(flags);
                match min {
                    Some(min) => buf.set_mc_f32(min),
                    _ => (),
                }
                match max {
                    Some(max) => buf.set_mc_f32(max),
                    _ => (),
                }
            }
            Integer { min, max } => {
                let mut flags = 0;
                if min.is_some() {
                    flags |= 1;
                }
                if max.is_some() {
                    flags |= 2;
                }
                buf.set_mc_u8(flags);
                match min {
                    Some(min) => buf.set_mc_i32(min),
                    _ => (),
                }
                match max {
                    Some(max) => buf.set_mc_i32(max),
                    _ => (),
                }
            }
            Long { min, max } => {
                let mut flags = 0;
                if min.is_some() {
                    flags |= 1;
                }
                if max.is_some() {
                    flags |= 2;
                }
                buf.set_mc_u8(flags);
                match min {
                    Some(min) => buf.set_mc_i64(min),
                    _ => (),
                }
                match max {
                    Some(max) => buf.set_mc_i64(max),
                    _ => (),
                }
            }
            Str(string_type) => {
                buf.set_mc_var_int(string_type as i32);
            }

            Entity {
                single,
                players_only,
            } => {
                let mut flags = 0;
                if single {
                    flags |= 1;
                }
                if players_only {
                    flags |= 2;
                }
                buf.set_mc_u8(flags);
            }
            ScoreHolder { multiple } => {
                let mut flags = 0;
                if multiple {
                    flags |= 1;
                }
                buf.set_mc_u8(flags);
            }
            _ => (),
        }
    }

    pub fn deserialize(name: String, buf: &mut BytesMut) -> Result<ArgumentType> {
        use ArgumentType::*;
        Ok(match &*name {
            "brigadier:bool" => Bool,
            "brigadier:double" => {
                let flags = buf.get_mc_u8()?;
                let min = if (flags & 1) > 0 {
                    Some(buf.get_mc_f64()?)
                } else {
                    None
                };
                let max = if (flags & 2) > 0 {
                    Some(buf.get_mc_f64()?)
                } else {
                    None
                };
                Double { min, max }
            }
            "brigadier:float" => {
                let flags = buf.get_mc_u8()?;
                let min = if (flags & 1) > 0 {
                    Some(buf.get_mc_f32()?)
                } else {
                    None
                };
                let max = if (flags & 2) > 0 {
                    Some(buf.get_mc_f32()?)
                } else {
                    None
                };
                Float { min, max }
            }
            "brigadier:integer" => {
                let flags = buf.get_mc_u8()?;
                let min = if (flags & 1) > 0 {
                    Some(buf.get_mc_i32()?)
                } else {
                    None
                };
                let max = if (flags & 2) > 0 {
                    Some(buf.get_mc_i32()?)
                } else {
                    None
                };
                Integer { min, max }
            }
            "brigadier:long" => {
                let flags = buf.get_mc_u8()?;
                let min = if (flags & 1) > 0 {
                    Some(buf.get_mc_i64()?)
                } else {
                    None
                };
                let max = if (flags & 2) > 0 {
                    Some(buf.get_mc_i64()?)
                } else {
                    None
                };
                Long { min, max }
            }
            "brigadier:string" => Str(buf.get_mc_enum()?),
            "minecraft:entity" => {
                let flags = buf.get_mc_u8()?;
                Entity {
                    single: (flags & 1) > 0,
                    players_only: (flags & 2) > 0,
                }
            }
            "minecraft:game_profile" => GameProfile,
            "minecraft:block_pos" => BlockPos,
            "minecraft:column_pos" => ColumnPos,
            "minecraft:vec3" => Vec3,
            "minecraft:vec2" => Vec2,
            "minecraft:block_state" => BlockState,
            "minecraft:block_predicate" => BlockPredicate,
            "minecraft:item_stack" => ItemStack,
            "minecraft:item_predicate" => ItemPredicate,
            "minecraft:color" => Color,
            "minecraft:component" => ChatComponent,
            "minecraft:message" => Message,
            "minecraft:nbt_compound_tag" => NbtCompoundTag,
            "minecraft:nbt_tag" => NbtTag,
            "minecraft:nbt_path" => NbtPath,
            "minecraft:objective" => Objective,
            "minecraft:objective_criteria" => ObjectiveCriteria,
            "minecraft:operation" => Operation,
            "minecraft:particle" => Particle,
            "minecraft:rotation" => Rotation,
            "minecraft:scoreboard_slot" => ScoreboardSlot,
            "minecraft:score_holder" => {
                let flags = buf.get_mc_u8()?;
                ScoreHolder {
                    multiple: (flags & 1) > 0,
                }
            }
            "minecraft:swizzle" => Swizzle,
            "minecraft:team" => Team,
            "minecraft:item_slot" => ItemSlot,
            "minecraft:resource_location" => ResourceLocation,
            "minecraft:mob_effect" => MobEffect,
            "minecraft:function" => Function,
            "minecraft:entity_anchor" => EntityAnchor,
            "minecraft:int_range" => IntRange,
            "minecraft:float_range" => FloatRange,
            "minecraft:item_enchantment" => ItemEnchantment,
            "minecraft:entity_summon" => EntitySummon,
            "minecraft:dimension" => Dimension,
            "minecraft:time" => Time,
            _ => return Err(Box::new(IoError::from(ErrorKind::InvalidData))),
        })
    }
}

struct TempCommandNode {
    command: bool,
    children: Vec<i32>,
    redirect: Option<i32>,
    data: Option<Either<(String, ArgumentType, Option<String>), String>>,
}

impl TempCommandNode {
    // doesn't follow "redirect"
    fn build(
        &self,
        index: i32,
        temp_nodes: &Vec<TempCommandNode>,
        output: &mut HashMap<i32, Arc<CommandNode>>,
    ) {
        if output.contains_key(&index) {
            return;
        }
        let base_node = BaseCommandNode {
            uuid: Uuid::new_v4(),
            children: LinkedHashMap::new(),
            redirect: None,
            command: self.command,
        };
        let node = match &self.data {
            None => {
                // root
                CommandNode::Root {
                    node: RwLock::new(base_node),
                }
            }
            Some(Either::Left((name, argument_type, custom_suggestions))) => {
                // argument
                CommandNode::Argument {
                    node: RwLock::new(base_node),
                    name: name.clone(),
                    argument_type: *argument_type,
                    custom_suggestions: custom_suggestions.clone(),
                }
            }
            Some(Either::Right(literal)) => {
                // literal
                CommandNode::Literal {
                    node: RwLock::new(base_node),
                    literal: literal.clone(),
                }
            }
        };
        let node = Arc::new(node);
        output.insert(index, node.clone());

        for child_index in &self.children {
            let temp_child = temp_nodes.get(*child_index as usize);
            if temp_child.is_none() {
                continue;
            }
            let temp_child = temp_child.unwrap();
            temp_child.build(*child_index, temp_nodes, output);
            let child_node = output[child_index].clone();
            let child_name = match &*child_node {
                CommandNode::Literal { literal, .. } => literal.clone(),
                CommandNode::Argument { name, .. } => name.clone(),
                _ => "".to_string(),
            };
            node.node_mut().children.insert(child_name, child_node);
        }
        node.node_mut().redirect = match self.redirect {
            Some(child_index) => {
                let temp_child = temp_nodes.get(child_index as usize);
                if temp_child.is_some() {
                    let temp_child = temp_child.unwrap();
                    temp_child.build(child_index, temp_nodes, output);
                    let child_node = output[&child_index].clone();
                    Some(child_node)
                } else {
                    None
                }
            }
            None => None,
        };
    }
}

impl CommandNode {
    fn mappify(
        self: Arc<CommandNode>,
    ) -> (
        HashMap<Uuid, usize>,
        Vec<Uuid>,
        HashMap<Uuid, Arc<CommandNode>>,
    ) {
        let mut output = HashMap::new();
        let mut all_nodes = HashMap::new();
        let mut deque: VecDeque<Arc<CommandNode>> = VecDeque::new();
        deque.push_back(self);
        while !deque.is_empty() {
            let node = deque.pop_front().unwrap();
            let base_node = node.node();
            output.insert(base_node.uuid, output.len());
            all_nodes.insert(base_node.uuid, node.clone());
            deque.extend(base_node.children.values().map(|value| value.clone()));
            if base_node.redirect.is_some() {
                deque.push_back(base_node.redirect.clone().unwrap());
            }
        }
        let mut output_vec: Vec<Uuid> = vec![];
        output_vec.reserve(output.len());
        for (node, i) in &output {
            output_vec[*i] = node.clone();
        }
        (output, output_vec, all_nodes)
    }

    fn serialize(self: Arc<CommandNode>, buf: &mut BytesMut, offsets: &HashMap<Uuid, usize>) {
        let base_node = self.node();
        let mut flags: u8 = 0;
        if base_node.redirect.is_some() {
            flags |= 8;
        }
        if base_node.command {
            flags |= 4;
        }
        match &*self {
            CommandNode::Root { .. } => {
                flags |= 0; // NOP
            }
            CommandNode::Argument {
                custom_suggestions, ..
            } => {
                flags |= 2;
                if custom_suggestions.is_some() {
                    flags |= 16;
                }
            }
            CommandNode::Literal { .. } => {
                flags |= 1;
            }
        }
        buf.set_mc_u8(flags);
        buf.set_mc_var_int(base_node.children.len() as i32);
        for (_, child) in &base_node.children {
            buf.set_mc_var_int(*offsets.get(&child.node().uuid).expect("malformed offsets") as i32);
        }

        if base_node.redirect.is_some() {
            buf.set_mc_var_int(
                *offsets
                    .get(&base_node.redirect.as_ref().unwrap().node().uuid)
                    .expect("malformed offsets") as i32,
            );
        }

        match &*self {
            CommandNode::Argument {
                name,
                argument_type,
                custom_suggestions,
                ..
            } => {
                buf.set_mc_string(name.clone());
                buf.set_mc_string(argument_type.name().to_string());
                argument_type.serialize(buf);
                match custom_suggestions {
                    Some(custom_suggestions) => {
                        buf.set_mc_string(custom_suggestions.clone());
                    }
                    _ => (),
                }
            }
            CommandNode::Literal { literal, .. } => {
                buf.set_mc_string(literal.clone());
            }
            _ => (),
        }
    }

    fn deserialize(buf: &mut BytesMut) -> Result<TempCommandNode> {
        let flags = buf.get_mc_u8()?;
        let redirect = (flags & 8) > 0;
        let command = (flags & 4) > 0;
        let is_argument = (flags & 2) > 0;
        let is_literal = (flags & 1) > 0;
        let has_suggestions = (flags & 16) > 0;
        let child_count = buf.get_mc_var_int()?;
        let mut children: Vec<i32> = vec![];
        for _ in 0..child_count {
            children.push(buf.get_mc_var_int()?);
        }
        let redirect = if redirect {
            Some(buf.get_mc_var_int()?)
        } else {
            None
        };
        let data = if is_argument {
            let name = buf.get_mc_string(32767)?;
            let argument_type = ArgumentType::deserialize(buf.get_mc_string(32767)?, buf)?;
            let custom_suggestions = if has_suggestions {
                Some(buf.get_mc_string(32767)?)
            } else {
                None
            };
            Some(Either::Left((name, argument_type, custom_suggestions)))
        } else if is_literal {
            Some(Either::Right(buf.get_mc_string(32767)?))
        } else {
            None
        };
        Ok(TempCommandNode {
            command,
            children,
            redirect,
            data,
        })
    }
}

pub struct CommandsPacket {
    pub root: Arc<CommandNode>,
}

impl CodablePacket for CommandsPacket {
    fn encode(self, buf: &mut BytesMut) {
        let (offsets, uuids, node_lookup) = self.root.clone().mappify();

        buf.set_mc_var_int(uuids.len() as i32);
        for uuid in uuids {
            node_lookup[&uuid].clone().serialize(buf, &offsets);
        }
        buf.set_mc_var_int(offsets[&self.root.node().uuid] as i32);
    }

    fn decode(buf: &mut BytesMut) -> Result<Self>
    where
        Self: Sized,
    {
        let node_count = buf.get_mc_var_int()?;
        let mut temp_nodes: Vec<TempCommandNode> = vec![];
        for _ in 0..node_count {
            temp_nodes.push(CommandNode::deserialize(buf)?);
        }
        let root_node_index = buf.get_mc_var_int()?;
        let root_temp_node = temp_nodes.get(root_node_index as usize);
        if root_temp_node.is_none() {
            return Err(Box::new(IoError::from(ErrorKind::InvalidData)));
        }
        let root_temp_node = root_temp_node.unwrap();
        let mut outmap = HashMap::new();
        root_temp_node.build(root_node_index, &temp_nodes, &mut outmap);
        let root = outmap.get(&root_node_index);
        if root.is_none() {
            return Err(Box::new(IoError::from(ErrorKind::InvalidData)));
        }
        return Ok(CommandsPacket {
            root: root.unwrap().clone(),
        });
    }
}
