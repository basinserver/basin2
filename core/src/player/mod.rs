use basin2_lib::result::*;
use async_trait::async_trait;
use basin2_protocol::network::*;
use basin2_protocol::packets::*;
use basin2_protocol::WrappedConnection;
use log::*;
use crate::util::{ MC_VERSION, CONFIG, PUBLIC_KEY };
use tokio::sync::Mutex;
use rand::prelude::*;
use basin2_lib::{ AtomicSet, Whitelist };
use openssl::rsa::Padding;
use bytes::BytesMut;
use bytes::buf::Buf;
use openssl::sha::sha1;
use serde::Deserialize;
use uuid::Uuid;
use linked_hash_map::LinkedHashMap;
use std::sync::Arc;
use crate::server::ServerT;
use futures::executor::block_on;
use crate::world::{ World, WorldT, Level, LevelT };
use basin2_lib::{ AtomicRef, Nbt };
use crate::entity::{ Entity, EntityT, player::* };
use std::sync::{ RwLock };
use enum_primitive::FromPrimitive;
use std::convert::TryFrom;
use basin2_data::{ DATA, BLOCKS, ITEMS, ENTITY_TYPES };
use std::sync::atomic::{ AtomicU32, Ordering };
use tokio::sync::oneshot;

#[derive(Clone)]
struct LoginState {
    started: bool,
    awaiting_response: bool,
    nonce: u32,
}

pub struct PlayerProperty {
    pub value: String,
    pub signature: Option<String>,
}

pub struct Player {
    pub server: ServerT,
    pub entity_id: u32,
    pub connection: WrappedConnection,
    pub disconnected: AtomicSet<bool>,
    pub username: AtomicSet<String>,
    pub uuid: AtomicSet<Uuid>,
    pub properties: AtomicSet<LinkedHashMap<String, PlayerProperty>>,
    login_state: Mutex<Option<LoginState>>,
    pub level: AtomicRef<dyn LevelT + 'static>,
    pub world: AtomicRef<dyn WorldT + 'static>,
    pub entity: AtomicRef<RwLock<EntityT>>,
    pub data: RwLock<Option<IngamePlayerData>>,
    pub teleport_id: AtomicU32,
}

pub const PLAYER_INVENTORY_SIZE: usize = 46;
pub const PLAYER_ENDER_SIZE: usize = 27;

#[derive(Clone)]
pub struct IngamePlayerData {
    pub dimension: DimensionType,
    pub game_type: GameType,
    pub selected_item_slot: i32,
    pub spawn: Option<(i32, i32, i32)>,
    pub spawn_forced: bool,
    pub food_level: i32,
    pub food_exhaustion_level: f32,
    pub food_saturation_level: f32,
    pub food_tick_timer: i32,
    pub xp_level: i32,
    pub xp_total: i32,
    pub xp_seed: i32,
    pub inventory: Vec<ItemStack>,
    pub ender_items: Vec<ItemStack>,
    pub seen_credits: bool,
    pub recipes: Vec<String>,
    pub walk_speed: f32,
    pub fly_speed: f32,
    pub may_fly: bool,
    pub flying: bool,
    pub invulnerable: bool,
    pub may_build: bool,
    pub instabuild: bool,
    pub is_op: bool,
}

impl IngamePlayerData {
    pub fn parse(nbt: &Nbt) -> Result<IngamePlayerData> {
        let dimension = DimensionType::from_i32(nbt.child("Dimension")?.unwrap_i32()?).ok_or(basin_err!("invalid dimension for player"))?;
        let game_type = GameType::from_i32(nbt.child("playerGameType")?.unwrap_i32()?).ok_or(basin_err!("invalid game type for player"))?;
        let spawn = if nbt.child("SpawnY").is_ok() {
            Some((
                nbt.child("SpawnX")?.unwrap_i32()?,
                nbt.child("SpawnY")?.unwrap_i32()?,
                nbt.child("SpawnZ")?.unwrap_i32()?,
            ))
        } else {
            None
        };
        let inventory_items = nbt.child("Inventory")?.unwrap_list()?;
        let mut inventory = vec![ItemStack::empty(); PLAYER_INVENTORY_SIZE];
        for item in inventory_items {
            let slot = item.child("Slot")?.unwrap_i8()?;
            let slot = match slot {
                0..=8 => slot + 36,
                9..=35 => slot,
                100..=103 => (3 - (slot - 100)) + 5,
                -106 => 45,
                _ => return Err(basin_err!("invalid slot value in player inventory: {}", slot)),
            };
            inventory[slot as usize] = ItemStack::try_from(item)?;
        }
        let ender_items_raw = nbt.child("EnderItems")?.unwrap_list()?;
        let mut ender_items = vec![ItemStack::empty(); PLAYER_ENDER_SIZE];
        for item in ender_items_raw {
            let slot = item.child("Slot")?.unwrap_i8()?;
            ender_items[slot as usize] = ItemStack::try_from(item)?;
        }
        let abilities = nbt.child("abilities")?;
        Ok(IngamePlayerData {
            dimension,
            game_type,
            selected_item_slot: nbt.child("SelectedItemSlot")?.unwrap_i32()?,
            spawn,
            spawn_forced: nbt.child("SpawnForced").and_then(|b| b.unwrap_i8()).map(|b| b == 1).unwrap_or(false),
            food_level: nbt.child("foodLevel")?.unwrap_i32()?,
            food_exhaustion_level: nbt.child("foodExhaustionLevel")?.unwrap_f32()?,
            food_saturation_level: nbt.child("foodSaturationLevel")?.unwrap_f32()?,
            food_tick_timer: nbt.child("foodTickTimer")?.unwrap_i32()?,
            xp_level: nbt.child("XpLevel")?.unwrap_i32()?,
            xp_total: nbt.child("XpTotal")?.unwrap_i32()?,
            xp_seed: nbt.child("XpSeed")?.unwrap_i32()?,
            inventory,
            ender_items,
            seen_credits: nbt.child("seenCredits")?.unwrap_i8()? == 1,
            recipes: nbt.child("recipeBook")?.child("recipes")?.unwrap_list()?.iter().map(|item| item.unwrap_str().map(|s| s.to_string()) ).collect::<Result<Vec<String>>>()?,
            walk_speed: abilities.child("walkSpeed")?.unwrap_f32()?,
            fly_speed: abilities.child("flySpeed")?.unwrap_f32()?,
            may_fly: abilities.child("mayfly")?.unwrap_i8()? == 1,
            flying: abilities.child("flying")?.unwrap_i8()? == 1,
            invulnerable: abilities.child("invulnerable")?.unwrap_i8()? == 1,
            may_build: abilities.child("mayBuild")?.unwrap_i8()? == 1,
            instabuild: abilities.child("instabuild")?.unwrap_i8()? == 1,
            is_op: false,
        })
    }
}

impl Drop for Player {
    fn drop(&mut self) {
        block_on(self.disconnect(ChatComponent::from("Disconnected"))).unwrap_or(());
    }
}

pub type PlayerT = Arc<Player>;

impl Player {
    pub async fn send_packet(&self, packet: Packet) -> Result<()> {
        let mut sender = self.connection.outgoing.lock().unwrap().clone();
        
        Ok(sender.send((None, Some(packet)))
            .await
            .map_err(|e| basin_err!("could not send packet: {:?}", e))?)
    }

    pub async fn send_packet_blocking(&self, packet: Packet) -> Result<()> {
        let mut sender = self.connection.outgoing.lock().unwrap().clone();
        
        let (response_sender, receiver) = oneshot::channel::<()>();

        let result = sender.send((Some(response_sender), Some(packet)))
            .await
            .map_err(|e| basin_err!("could not send packet: {:?}", e) as Error);
        
        receiver.await.unwrap();
        result
    }

    pub async fn disconnect(&self, reason: ChatComponent) -> Result<()> {
        self.disconnected.try_set(true);
        {
            let codec_state = self.connection.codec.0.read().unwrap().state;
            if codec_state == ConnectionProtocol::Login {
                self.send_packet(Packet::from(Login::from(LoginDisconnectPacket { reason })))
                    .await?;
            } else if codec_state == ConnectionProtocol::Game {
                self.send_packet(Packet::from(Game::from(DisconnectPacket { reason })))
                    .await?;
            }
        }
        let mut sender = self.connection.outgoing.lock().unwrap().clone();
        
        Ok(sender.send((None, None))
            .await
            .map_err(|e| basin_err!("could not send packet: {:?}", e))?)
    }
}

#[async_trait]
pub trait ProtocolHandler {
    async fn handle_handshake(self: &Arc<Self>, packet: &HandshakeServerbound) -> Result<()>;
    async fn handle_login(self: &Arc<Self>, packet: &LoginServerbound) -> Result<()>;
    async fn handle_status(self: &Arc<Self>, packet: &StatusServerbound) -> Result<()>;
    async fn handle_game(self: &Arc<Self>, packet: &GameServerbound) -> Result<()>;
}

#[derive(Deserialize)]
struct SessionResponseProperty {
    name: String,
    value: String,
    signature: String,
}

#[derive(Deserialize)]
struct SessionResponse {
    id: Uuid,
    name: String,
    properties: Vec<SessionResponseProperty>,
}

lazy_static! {
    pub static ref VALID_USERNAME: Vec<char> = { "qwertyuiopasdfghjklzxcvbnmQWERTYUIOPASDFGHJKLZXCVBNM1234567890_".chars().collect() };
}

impl Player {

    async fn teleport_sync(self: &PlayerT) -> Result<()> {
        let formed_packet = {
            let id = self.teleport_id.fetch_add(1, Ordering::Relaxed) + 1;
            let entity = self.entity.read().unwrap();
            PlayerPositionPacket {
                id: id as i32,
                x: entity.pos.x,
                y: entity.pos.y,
                z: entity.pos.z,
                yRot: entity.rot.yaw,
                xRot: entity.rot.pitch,
                relativeArguments: (false, false, false, false, false),
            }
        };
        self.send_packet(Packet::from(Game::from(formed_packet))).await?;
        Ok(())
    }

    async fn finish_login(self: &PlayerT) -> Result<()> {
        match CONFIG.compression_threshold {
            Some(threshold) => {
                self.send_packet(Packet::from(Login::from(LoginCompressionPacket { compressionThreshold: threshold as i32 })))
                .await?;
                self.connection.set_compression(threshold);
            }
            _ => ()
        }
        self.send_packet_blocking(Packet::from(Login::from(GameProfilePacket { gameProfile: GameProfile {
            uuid: Some(*self.uuid),
            name: (*self.username).clone(),
            legacy: false,
        } })))
        .await?;

        self.connection.set_state(ConnectionProtocol::Game);

        let entity_data = self.server.level.player_data(&self.uuid);

        let data = entity_data.as_ref().map(|data| IngamePlayerData::parse(data)).unwrap_or_else(|| {
            Ok(IngamePlayerData {
                dimension: DimensionType::Overworld,
                game_type: GameType::try_from(&*CONFIG.game_type)?,
                selected_item_slot: 0,
                spawn: None,
                spawn_forced: false,
                food_level: 20,
                food_exhaustion_level: 0.0,
                food_saturation_level: 0.0,
                food_tick_timer: 0,
                xp_level: 0,
                xp_total: 0,
                xp_seed: 0,
                inventory: vec![ItemStack::empty(); PLAYER_INVENTORY_SIZE],
                ender_items: vec![ItemStack::empty(); PLAYER_ENDER_SIZE],
                seen_credits: false,
                recipes: vec![],
                walk_speed: 0.1,
                fly_speed: 0.05,
                may_fly: false,
                flying: false,
                invulnerable: false,
                may_build: true,
                instabuild: false,
                is_op: false,
            })
        })?;

        // self.server.level.set_player_data(&self.uuid, &data);
        {
            *self.data.write().unwrap() = Some(data);
        }
        let game_type = {
            self.data.read().unwrap().as_ref().unwrap().game_type
        };

        {
            let dimension = { self.data.read().unwrap().as_ref().unwrap().dimension };
            let world = self.server.level.dimensions().get(&dimension.id()).ok_or(basin_err!("dimension not found for player: {}", dimension.id()))?;
            self.world.set(world.clone());
        }
        let player_entity = Arc::new(RwLock::new({
            if let Some(entity_data) = entity_data {
                EntityT::try_from(self.world.get(), entity_data, Some(Box::new(PlayerData { player: self.clone() })))?
            } else {
                new_player_entity(self.world.get(), self)?
            }
        }));

        self.entity.set(player_entity.clone());

        let formed_packet = {
            let player_entity = self.entity.get();
            let player_entity = player_entity.read().unwrap();
            LoginPacket {
                playerId: player_entity.id as i32, // TODO
                seed: self.server.level.seed() as i64,
                hardcore: CONFIG.hardcore,
                gameType: game_type,
                dimension: player_entity.world.dimension(),
                maxPlayers: CONFIG.max_players as u8,
                levelType: "default".to_string(),
                chunkRadius: CONFIG.view_distance as i32,
                reducedDebugInfo: self.server.level.game_rules().get("reducedDebugInfo").map(|value| *value == "true").unwrap_or(false),
                showDeathScreen: self.server.level.game_rules().get("doImmediateRespawn").map(|value| *value == "true").unwrap_or(false),
            }
        };
        self.send_packet(Packet::from(Game::from(formed_packet))).await?;
        self.send_packet(Packet::from(Game::from(game::clientbound::CustomPayloadPacket {
            identifier: "brand".to_string(),
            data: BytesMut::from("basin2".as_bytes()),
        }))).await?;
        let (difficulty, locked) = self.server.level.difficulty();
        self.send_packet(Packet::from(Game::from(game::clientbound::ChangeDifficultyPacket {
            difficulty,
            locked,
        }))).await?;
        let formed_packet = {
            let data = self.data.read().unwrap();
            let data = data.as_ref().unwrap();
            game::clientbound::PlayerAbilitiesPacket {
                walkingSpeed: data.walk_speed,
                flyingSpeed: data.fly_speed,
                invulnerable: data.invulnerable,
                isFlying: data.flying,
                canFly: data.may_fly,
                instabuild: data.instabuild,
            }
        };
        self.send_packet(Packet::from(Game::from(formed_packet))).await?;
        self.send_packet(Packet::from(Game::from(game::clientbound::SetCarriedItemPacket {
            slot: 0,
        }))).await?;
        self.send_packet(Packet::from(Game::from(UpdateRecipesPacket {
            recipes: DATA.recipes.iter().map(|(key, value)| (key.clone(), value.clone())).collect(),
        }))).await?;
        self.send_packet(Packet::from(Game::from(UpdateTagsPacket {
            blocks: DATA.tags_blocks.iter().map(|(key, value)| (key.clone(), value.values.iter().map(|value| {
                BLOCKS.get_str(value).map(|block| block.registry_id.load(Ordering::Relaxed) as i32 ).unwrap_or(0)
            }).collect())).collect(),
            items: DATA.tags_items.iter().map(|(key, value)| (key.clone(), value.values.iter().map(|value| {
                ITEMS.get_str(value).map(|item| item.registry_id.load(Ordering::Relaxed) as i32 ).unwrap_or(0)
            }).collect())).collect(),
            entityTypes: DATA.tags_entity_types.iter().map(|(key, value)| (key.clone(), value.values.iter().map(|value| {
                ENTITY_TYPES.get_str(value).map(|entity_type| entity_type.registry_id.load(Ordering::Relaxed) as i32 ).unwrap_or(0)
            }).collect())).collect(),
            fluids: vec![], // TODO
        }))).await?;
        let permission_level: u8 = {
            let data = self.data.read().unwrap();
            let data = data.as_ref().unwrap();
            if data.is_op {
                28
            } else {
                24
            }
        };
        self.send_packet(Packet::from(Game::from(EntityEventPacket {
            entityId: self.entity_id as i32,
            eventId: permission_level,
        }))).await?;
        self.send_packet(Packet::from(Game::from(CommandsPacket {
            root: self.server.commands.clone(),
        }))).await?;
        let formed_packet = {
            let data = self.data.read().unwrap();
            let data = data.as_ref().unwrap();
            RecipePacket {
                state: RecipePacketState::Init,
                recipes: data.recipes.clone(),
                toHighlight: Some(vec![]),
                guiOpen: false,
                filteringCraftable: false,
                furnaceGuiOpen: false,
                furnaceFilteringCraftable: false,
            }
        };
        self.send_packet(Packet::from(Game::from(formed_packet))).await?;
        //TODO: scoreboard sync

        //TODO: send entity to world, spawn logic, etc

        self.teleport_sync().await?;

        info!("Player {} has joined!", *self.username);

        //TODO: broadcast join message, send player info packets, level info, resource pack
        //self.disconnect(ChatComponent::from("test successful".to_string())).await?;
        Ok(())
    }

    async fn authenticate_mojang(&self, shared_secret: &[u8]) -> Result<bool> {
        let mut buf = BytesMut::new();
        buf.extend_from_slice(shared_secret);
        buf.extend_from_slice(&PUBLIC_KEY.1[..]);
        let mut raw_hash = sha1(&buf[..]);
        let signed = (raw_hash[0] & 0x80) != 0;
        if signed {
            for i in 0..20 {
                raw_hash[i] = !raw_hash[i];
            }
            raw_hash[19] += 1;
        }
        let raw_hash = raw_hash.to_vec().iter().map(|b| format!("{:02x}", b)).collect::<Vec<String>>().join("");
        let mut leading_zero_count = 0;
        for c in raw_hash.chars() {
            if c == '0' {
                leading_zero_count += 1;
            } else {
                break;
            }
        }
        let hash = format!("{}{}", if signed { "-" } else { "" }, &raw_hash[leading_zero_count..]);
        let client = reqwest::Client::new();
        let mut response = client
            .get(&*format!("https://sessionserver.mojang.com/session/minecraft/hasJoined?username={}&serverId={}", *self.username, hash))
            .send()?;
        if response.status().as_u16() != 200 {
            return Ok(false);
        }
        let body: SessionResponse = response.json()?;
        if body.name != *self.username {
            return Ok(false);
        }
        self.uuid.set(body.id);
        let mut properties: LinkedHashMap<String, PlayerProperty> = LinkedHashMap::new();
        for property in body.properties {
            properties.insert(property.name, PlayerProperty { value: property.value, signature: Some(property.signature) });
        }
        self.properties.set(properties);
        Ok(true)
    }

}

lazy_static! {
    pub static ref USER_NAMESPACE: Uuid = { Uuid::parse_str("033e0831-0690-4731-bfe5-9eacd9c54ade").unwrap() };
}

#[async_trait]
impl ProtocolHandler for Player {
    async fn handle_handshake(self: &PlayerT, packet: &HandshakeServerbound) -> Result<()> {
        use HandshakeServerbound::*;

        match packet {
            ClientIntentionPacket(packet) => {
                if basin2_protocol::PROTOCOL_VERSION != packet.protocolVersion as u32 {
                    return Err(basin_err!("invalid protocol version: {}", packet.protocolVersion));
                }
                if packet.intention == ConnectionProtocol::Status {
                    self.connection.set_state(ConnectionProtocol::Status);
                } else if packet.intention == ConnectionProtocol::Login {
                    self.connection.set_state(ConnectionProtocol::Login);
                } else {
                    return Err(basin_err!("invalid target: {:?}", packet.intention));
                }
                Ok(())
            },
            _ => Err(basin_err!("invalid packet: {:?}", packet)),
        }
    }

    async fn handle_login(self: &PlayerT, packet: &LoginServerbound) -> Result<()> {
        use LoginServerbound::*;

        let mut login_state = self.login_state.lock().await;
        if login_state.is_none() {
            login_state.replace(LoginState {
                started: false,
                awaiting_response: false,
                nonce: rand::thread_rng().gen::<u32>(),
            });
        }
        match packet {
            HelloPacket(packet) => {
                if login_state.as_ref().unwrap().started {
                    return Err(basin_err!("client sent hello packet twice"));
                }
                if !packet.username.chars().into_iter().whitelist(&*VALID_USERNAME) {
                    return Err(basin_err!("invalid usernme"));
                }
                self.username.set(packet.username.clone());
                let login_state = login_state.as_mut().unwrap();
                login_state.started = true;
                if CONFIG.online_mode {
                    login_state.awaiting_response = true;
                    self.send_packet(Packet::from(LoginClientbound::from(basin2_protocol::packets::login::clientbound::HelloPacket {
                        serverId: "".to_string(),
                        publicKey: PUBLIC_KEY.1.to_vec(),
                        nonce: login_state.nonce.to_be_bytes().to_vec(),
                    })))
                    .await?;
                } else {
                    login_state.awaiting_response = false;
                    self.uuid.set(Uuid::new_v5(&*USER_NAMESPACE, &**self.username));
                    self.properties.set(LinkedHashMap::new());
                    self.clone().finish_login().await?;
                }
                Ok(())
            },
            KeyPacket(packet) => {
                let login_state = login_state.as_mut().unwrap();
                if !login_state.awaiting_response {
                    return Err(basin_err!("responded to non-request of encryption"));
                }
                if packet.keybytes.len() > 256 || packet.nonce.len() > 256 {
                    return Err(basin_err!("keys too long got: {}/{}, expecting: <=256/<=256", packet.keybytes.len(), packet.nonce.len()));
                }
                let mut shared_secret = vec![0; 256];
                let total_decrypted = PUBLIC_KEY.0.private_decrypt(&packet.keybytes, &mut shared_secret, Padding::PKCS1)?;
                if total_decrypted != 16 {
                    return Err(basin_err!("shared secret not 16 bytes: {}", total_decrypted));
                }
                let shared_secret = &shared_secret[0..16];
                let mut nonce = vec![0; 256];
                let total_decrypted = PUBLIC_KEY.0.private_decrypt(&packet.nonce, &mut nonce, Padding::PKCS1)?;
                if total_decrypted != 4 {
                    return Err(basin_err!("nonce not 4 bytes: {}", total_decrypted));
                }
                let nonce = &nonce[0..4];
                let nonce = BytesMut::from(&nonce[..]).get_u32();
                if nonce != login_state.nonce {
                    return Err(basin_err!("nonce did not match", ));
                }
                self.connection.init_encryption(&shared_secret[..])?;

                if !self.authenticate_mojang(&shared_secret[..]).await? {
                    return Err(basin_err!("bad login"));
                }
                self.clone().finish_login().await?;
                Ok(())
            },
            CustomQueryPacket(_packet) => {
                Err(basin_err!("unexpected query response in login"))
            },
            _ => Err(basin_err!("invalid packet: {:?}", packet)),
        }
    }

    async fn handle_status(self: &PlayerT, packet: &StatusServerbound) -> Result<()> {
        use StatusServerbound::*;
        
        match packet {
            StatusRequestPacket(_packet) => {
                self.send_packet(Packet::from(Status::from(StatusResponsePacket {
                    status: ServerStatus {
                        description: ChatComponent::from(&*CONFIG.server_description).serialize(),
                        players: ServerStatusPlayers {
                            max: CONFIG.max_players as i32,
                            online: 3,
                            sample: None,
                        },
                        version: ServerStatusVersion {
                            name: (&*MC_VERSION).clone(),
                            protocol: basin2_protocol::PROTOCOL_VERSION as i32,
                        },
                        favicon: None,
                    },
                })))
                .await?;
                Ok(())
            },
            PingRequestPacket(packet) => {
                self.send_packet(Packet::from(Status::from(PongResponsePacket {
                    time: packet.time,
                })))
                .await?;
                Ok(())
            },
            _ => Err(basin_err!("invalid packet: {:?}", packet)),
        }
    }

    async fn handle_game(self: &PlayerT, packet: &GameServerbound) -> Result<()> {
        println!("{:?}", packet);
        Ok(())
    }
}

pub async fn handle_connection(connection: WrappedConnection, server: ServerT) {
    // lock the mutex then claim the receiver for ourselves
    let mut incoming = connection.incoming.lock().unwrap().take().unwrap();
    let player = Arc::new(Player {
        server: server.clone(),
        entity_id: server.level.next_entity_id(),
        connection,
        login_state: Mutex::new(None),
        username: AtomicSet::new(),
        uuid: AtomicSet::new(),
        properties: AtomicSet::new(),
        disconnected: AtomicSet::new(),
        level: AtomicRef::from(server.level.clone()),
        world: AtomicRef::new(),
        entity: AtomicRef::new(),
        data: RwLock::new(None),
        teleport_id: AtomicU32::new(0),
    });
    while let Some((finish, packet)) = incoming.recv().await {
        let result = match packet {
            Packet::Handshake(Handshake::HandshakeServerbound(packet)) => {
                player.handle_handshake(&packet).await
            }
            Packet::Login(Login::LoginServerbound(packet)) => player.handle_login(&packet).await,
            Packet::Status(Status::StatusServerbound(packet)) => player.handle_status(&packet).await,
            Packet::Game(Game::GameServerbound(packet)) => player.handle_game(&packet).await,
            _ => {
                error!("received invalid packet {:?}", packet);
                if let Some(finish) = finish {
                    finish.send(()).unwrap_or(());
                }
                break;
            }
        };
        if let Some(finish) = finish {
            finish.send(()).unwrap_or(());
        }
        if result.is_err() {
            error!(
                "error handling packet from player: {:?}",
                result.err().unwrap()
            );
            player.disconnect(ChatComponent::from("Disconnected. Ask your server admin for more details.")).await.unwrap_or(());
            break;
        }
    }
    player.disconnect(ChatComponent::from("Disconnect")).await.unwrap_or(());
}
