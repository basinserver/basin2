
const fs = require('fs');
const { execSync } = require('child_process');
const protocol = JSON.parse(fs.readFileSync(process.argv[2], 'UTF-8'));

const makeAggregrateModule = children => children.map(child => `pub mod ${child};\npub use ${child}::*;\n`).join('');

const camelToSnake = camel => {
    let output = [];
    for (let i in camel) {
        const c = camel[i];
        if (c.toUpperCase() == c) {
            if (i > 0) output.push('_');
            output.push(c.toLowerCase());
        } else {
            output.push(c);
        }
    }
    return output.join('');
}

const simpleCamel = snake => snake[0].toUpperCase() + snake.substring(1);

const declarationTypeMap = {
    'int': 'i32',
    'BlockPos': 'BlockPos',
    'String': 'String',
    'boolean': 'bool',
    'short': 'i16',
    'ItemStack': 'ItemStack',
    'ResourceLocation': 'ResourceLocation',
    'FriendlyByteBuf': 'BytesMut',
    'long': 'i64',
    'double': 'f64',
    'float': 'f32',
    'String[]': 'Vec<String>',
    'UUID': 'Uuid',
    'BlockHitResult': 'BlockHitResult',
    'byte': 'u8',
    'Component': 'ChatComponent',
    'Item': 'Item',
    'byte[]': 'Vec<u8>',
    'int[]': 'Vec<i32>',
    'CompoundTag': 'Nbt',
    'Difficulty': 'Difficulty',
    'ChatVisiblity': 'ChatVisiblity',
    'HumanoidArm': 'HumanoidArm',
    'ClickType': 'ClickType',
    'InteractionHand': 'InteractionHand',
    'Direction': 'Direction',
    'StructureMode': 'StructureMode',
    'Mirror': 'Mirror',
    'Rotation': 'Rotation',
    'ChatType': 'ChatType',
    'SoundSource': 'SoundSource',
    'GameType': 'GameType',
    'EquipmentSlot': 'EquipmentSlot',
    'ChatFormatting': 'ChatFormatting',
    'ConnectionProtocol': 'ConnectionProtocol',
    'CommandBlockEntity.Mode': 'CommandBlockEntityMode',
    'StructureBlockEntity.UpdateType': 'StructureBlockEntityUpdateType',
    'BossEvent.BossBarColor': 'BossBarColor',
    'BossEvent.BossBarOverlay': 'BossBarOverlay',
    'EntityAnchorArgument.Anchor': 'EntityAnchor',
    'ObjectiveCriteria.RenderType': 'ObjectiveCriteriaRenderType',
    'ServerScoreboard.Method': 'ServerScoreboardMethod',
    'ServerboundClientCommandPacket.Action': 'ClientCommandPacketAction',
    'ServerboundInteractPacket.Action': 'InteractPacketAction',
    'ServerboundPlayerActionPacket.Action': 'PlayerActionPacketAction',
    'ServerboundPlayerCommandPacket.Action': 'PlayerCommandPacketAction',
    'ServerboundRecipeBookUpdatePacket.Purpose': 'RecipeBookUpdatePacketPurpose',
    'ServerboundResourcePackPacket.Action': 'ResourcePackPacketAction',
    'ServerboundSeenAdvancementsPacket.Action': 'SeenAdvancementsPacketAction',
    'ClientboundBossEventPacket.Operation': 'BossEventPacketOperation',
    'ClientboundPlayerCombatPacket.Event': 'PlayerCombatPacketEvent',
    'ClientboundPlayerInfoPacket.Action': 'PlayerInfoPacketAction',
    'ClientboundRecipePacket.State': 'RecipePacketState',
    'ClientboundSetBorderPacket.Type': 'SetBorderPacketType',
    'ClientboundSetTitlesPacket.Type': 'SetTitlesPacketType',
    'ChunkBiomeContainer': 'Vec<i32>',
    'DimensionType': 'String',
    'LevelType': 'String',
    'MapDecoration[]': 'Vec<MapDecoration>',
    'MerchantOffers': 'Vec<MerchantOffer>',
    'MobEffect': 'MobEffect',
    'SoundEvent': 'SoundEvent',
    'GameProfile': 'GameProfile',
    'PublicKey': 'Vec<u8>',
    'ServerStatus': 'ServerStatus',
    'ClientboundChunkBlocksUpdatePacket.BlockUpdate[]': 'Vec<ChunkBlocksUpdatePacketBlockUpdate>',
    'BlockState': 'BlockState',
    'EntityType': 'EntityType',
};

const opTypeMap = {
    'VarInt': 'mc_var_int',
    'BlockPos': 'mc_block_pos',
    'Utf': 'mc_string',
    'Byte': 'mc_u8',
    'Boolean': 'mc_bool',
    'UnsignedByte': 'mc_u8',
    'Short': 'mc_i16',
    'Item': 'mc_item_stack',
    'Long': 'mc_i64',
    'Double': 'mc_f64',
    'Float': 'mc_f32',
    'ResourceLocation': 'mc_string',
    'VarLong': 'mc_i64',
    'UUID': 'mc_uuid',
    'BlockHitResult': 'mc_block_hit_result',
    'Int': 'mc_i32',
    'Nbt': 'mc_nbt',
    'Component': 'mc_chat_component',
    'VarIntArray': 'mc_var_int_array',
    'UnsignedShort': 'mc_u16',
    'ByteArray': 'mc_byte_array',
    'Enum': 'mc_var_int',
};

const outDir = process.argv[3];
execSync(`rm -rf "${outDir}"`);

const readOpEncode = readOp => {
    if (readOp.type == 'extra') {
        return `// TODO: EXTRA: ${readOp.raw}`;
    } else if (readOp.type == 'raw') {
        return `// TODO: UNKNOWN: ${readOp.raw}`;
    } else if (readOp.type == 'Utf') {
        return `let ${readOp.field} = buf.get_mc_string(${readOp.max})?;`;
    } else {
        return `let ${readOp.field} = buf.get_${opTypeMap[readOp.type]}()?;`;
    }
};

const writeOpEncode = writeOp => {
    if (writeOp.type == 'extra') {
        return `// TODO: EXTRA: ${writeOp.raw}`;
    } else if (writeOp.type == 'raw') {
        return `// TODO: UNKNOWN: ${writeOp.raw}`;
    } else {
        return `buf.set_${opTypeMap[writeOp.type]}(self.${writeOp.field});`;
    }
};

Object.entries(protocol).forEach(([stage, scopes]) => {
    Object.entries(scopes).forEach(([scopeName, scope]) => {
        const localOutDir = `${outDir}/${stage}/${scopeName}`;
        execSync(`mkdir -p "${localOutDir}"`);
        for (const packetId in scope) {
            const packet = scope[packetId];
            if (packet == null) {
                continue; // not implemented id
            }
            const moduleName = camelToSnake(packet.name);
            const module = `
use crate::packet::*;
use crate::network::*;
use bytes::BytesMut;
use uuid::Uuid;
use crate::result::*;

pub struct ${packet.name} {
    ${Object.entries(packet.struct).map(([name, type]) => `pub ${name}: ${declarationTypeMap[type]},`).join("\n    ")}
}

impl CodablePacket for ${packet.name} {
    fn encode(self, buf: &mut BytesMut) {
        ${packet.writeOps == null ? '/* TODO: NOT FOUND */' : packet.writeOps.map(writeOpEncode).join('\n        ')}
    }

    fn decode(buf: &mut BytesMut) -> Result<Self> where Self: Sized {
        ${packet.readOps == null ? '/* TODO: NOT FOUND */' : packet.readOps.map(readOpEncode).join('\n        ')}
        return Ok(${packet.name} { ${Object.keys(packet.struct).join(', ')} });
    }
}
`;
            fs.writeFileSync(`${localOutDir}/${moduleName}.rs`, module);
        }
        const aggModule = makeAggregrateModule(scope.filter(packet => packet != null).map(packet => camelToSnake(packet.name)));
        const packetEnum = `Packet${simpleCamel(stage)}${simpleCamel(scopeName)}`;
        fs.writeFileSync(`${localOutDir}/mod.rs`,
`${aggModule}
use bytes::BytesMut;
use crate::Result;
use std::io::Error as IoError;
use std::io::ErrorKind;
use crate::packet::*;
use crate::network::*;

pub fn decode_packet(id: i32, buf: &mut BytesMut) -> Result<${packetEnum}> {
    match id {
        ${scope.map((packet, id) => packet == null ? `${id} => Err(/* TODO: NYI */)` : `${id} => Ok(${packetEnum}::${packet.name}(${packet.name}::decode(buf)?)),`).join("\n        ")}
        _ => Err(Box::new(IoError::from(ErrorKind::InvalidData)))
    }
}

pub fn encode_packet(packet: ${packetEnum}, buf: &mut BytesMut) {
    match packet {
        ${scope.map((packet, id) => packet == null ? `null => Err(/* TODO: NYI */)` : `${packetEnum}::${packet.name}(deref_packet) => { buf.set_mc_var_int(${id}); deref_packet.encode(buf); },`).join("\n        ")}
    }
}

pub enum ${packetEnum} {
    ${scope.map((packet, id) => packet == null ? `/* TODO: NYI */` : `${packet.name}(${packet.name}),`).join("\n    ")}
}

`);
    });
    const packetStageEnum = `Packet${simpleCamel(stage)}`;
    const stageAggModule = makeAggregrateModule(Object.keys(scopes));
    fs.writeFileSync(`${outDir}/${stage}/mod.rs`,
`${stageAggModule}
pub enum ${packetStageEnum} {
    ${Object.keys(scopes).map(scopeName => `${packetStageEnum}${simpleCamel(scopeName)}(${packetStageEnum}${simpleCamel(scopeName)}),`).join("\n    ")}
}
`);
});
const globalAggModule = makeAggregrateModule(Object.keys(protocol));
fs.writeFileSync(`${outDir}/mod.rs`,
`${globalAggModule}
pub enum Packet {
    ${Object.keys(protocol).map(stage => `Packet${simpleCamel(stage)}(Packet${simpleCamel(stage)}),`).join("\n    ")}
}
`);
