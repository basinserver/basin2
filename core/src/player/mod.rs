use crate::result::*;
use async_trait::async_trait;
use basin2_protocol::network::*;
use basin2_protocol::packets::*;
use basin2_protocol::WrappedConnection;
use log::*;
use crate::util::{ MC_VERSION, CONFIG, PUBLIC_KEY };
use tokio::sync::Mutex;
use rand::prelude::*;
use crate::util::{ AtomicSet, Whitelist };
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
    pub connection: WrappedConnection,
    pub disconnected: AtomicSet<bool>,
    pub username: AtomicSet<String>,
    pub uuid: AtomicSet<Uuid>,
    pub properties: AtomicSet<LinkedHashMap<String, PlayerProperty>>,
    login_state: Mutex<Option<LoginState>>,
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
        
        Ok(sender.send(Some(packet))
            .await
            .map_err(|e| basin_err!("could not send packet: {:?}", e))?)
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
        
        Ok(sender.send(None)
            .await
            .map_err(|e| basin_err!("could not send packet: {:?}", e))?)
    }
}

#[async_trait]
pub trait ProtocolHandler {
    async fn handle_handshake(&self, packet: &HandshakeServerbound) -> Result<()>;
    async fn handle_login(&self, packet: &LoginServerbound) -> Result<()>;
    async fn handle_status(&self, packet: &StatusServerbound) -> Result<()>;
    async fn handle_game(&self, packet: &GameServerbound) -> Result<()>;
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

    async fn finish_login(&self) -> Result<()> {
        match CONFIG.compression_threshold {
            Some(threshold) => {
                self.send_packet(Packet::from(Login::from(LoginCompressionPacket { compressionThreshold: threshold as i32 })))
                .await?;
                self.connection.set_compression(threshold);
            }
            _ => ()
        }
        self.send_packet(Packet::from(Login::from(GameProfilePacket { gameProfile: GameProfile {
            uuid: Some(*self.uuid),
            name: self.username.clone(),
            legacy: false,
        } })))
        .await?;
        self.connection.set_state(ConnectionProtocol::Game);

        info!("Player {} has joined!", *self.username);
        self.disconnect(ChatComponent::from("test successful".to_string())).await?;
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
    async fn handle_handshake(&self, packet: &HandshakeServerbound) -> Result<()> {
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

    async fn handle_login(&self, packet: &LoginServerbound) -> Result<()> {
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
                    self.finish_login().await?;
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
                self.finish_login().await?;
                Ok(())
            },
            CustomQueryPacket(_packet) => {
                Err(basin_err!("unexpected query response in login"))
            },
            _ => Err(basin_err!("invalid packet: {:?}", packet)),
        }
    }

    async fn handle_status(&self, packet: &StatusServerbound) -> Result<()> {
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

    async fn handle_game(&self, packet: &GameServerbound) -> Result<()> {
        Ok(())
    }
}

pub async fn handle_connection(connection: WrappedConnection, server: ServerT) {
    // lock the mutex then claim the receiver for ourselves
    let mut incoming = connection.incoming.lock().unwrap().take().unwrap();
    let player = Arc::new(Player {
        server,
        connection,
        login_state: Mutex::new(None),
        username: AtomicSet::new(),
        uuid: AtomicSet::new(),
        properties: AtomicSet::new(),
        disconnected: AtomicSet::new(),
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
                break;
            }
        };
        if result.is_err() {
            error!(
                "error receiving packet from player: {:?}",
                result.err().unwrap()
            );
            player.disconnect(ChatComponent::from("Disconnected. Ask your server admin for more details.")).await.unwrap_or(());
            break;
        }
        match finish {
            Some(finish) => finish.send(()).unwrap_or(()),
            _ => (),
        }
    }
    player.disconnect(ChatComponent::from("Disconnect")).await.unwrap_or(());
}
