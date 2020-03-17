use super::*;

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
        /*
        TODO: from update recipes packet:
    java.lang.IndexOutOfBoundsException: index: 1, length: 98 (expected: range(0, 6))
        at io.netty.buffer.AbstractByteBuf.checkIndex0(AbstractByteBuf.java:1362)
        at io.netty.buffer.AbstractByteBuf.checkIndex(AbstractByteBuf.java:1357)
        at io.netty.buffer.PooledUnsafeDirectByteBuf.nioBuffer(PooledUnsafeDirectByteBuf.java:324)
        at io.netty.buffer.ByteBufUtil.decodeString(ByteBufUtil.java:769)
        at io.netty.buffer.AbstractByteBuf.toString(AbstractByteBuf.java:1222)
        at kv.toString(SourceFile:1292)
        at kv.e(SourceFile:372)
        at dnp.a(SourceFile:1789)
        at mw.a(SourceFile:65)
        at mw.a(SourceFile:10)
        at lv.a(SourceFile:21)
        at ais.c(SourceFile:144)
        at aiw.c(SourceFile:23)
        at ais.w(SourceFile:118)
        at ais.bk(SourceFile:103)
        at dbn.d(SourceFile:956)
        at dbn.d(SourceFile:619)
        at net.minecraft.client.main.Main.main(SourceFile:204)
        */
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

        // technically a race condition here with the main player dispatch loop, but no effect expected
        self.run_loops();

        let player_info_data = {
            let mut player_info_data: Vec<(Uuid, PlayerInfoPacketData)> = vec![];
            for (uuid, player) in self.server.players.read().unwrap().iter() {
                player_info_data.push((
                    uuid.clone(),
                    PlayerInfoPacketData::AddPlayer(
                        (*player.username).clone(),
                        (*player.properties).clone(),
                        player.data.read().unwrap().as_ref().unwrap().game_type,
                        0,
                        None,
                    ),
                ))
            }
            player_info_data
        };
        self.send_packet(Packet::from(Game::from(PlayerInfoPacket {
            action: PlayerInfoPacketAction::AddPlayer,
            data: player_info_data,
        }))).await?;

        {
            self.server.players.write().unwrap().insert(*self.uuid, self.clone());
        }
        
        self.server.send(PlayerJoinEvent {
            from: self.clone(),
        });

        self.server.send(ChatBroadcastEvent {
            chat_type: ChatType::System,
            component: ChatComponent::from(format!("Player {} has joined!", *self.username)),
        });

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
        let mut properties = vec![];
        for property in body.properties {
            properties.push(PlayerProperty { name: property.name, value: property.value, signature: Some(property.signature) });
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
                    self.properties.set(vec![]);
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
        self.handle_game_packet(packet).await
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
        last_ping: AtomicU64::new(0),
        latency: AtomicU64::new(0),
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
