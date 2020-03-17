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
use std::sync::atomic::{ AtomicU32, AtomicU64, Ordering };
use tokio::sync::{ oneshot, broadcast };
use tokio::time::delay_for;
use std::time::Duration;
use crate::util::epoch;

mod login;
mod data;
mod event;
pub use login::handle_connection;
pub use data::*;
pub use event::*;

#[derive(Clone)]
struct LoginState {
    started: bool,
    awaiting_response: bool,
    nonce: u32,
}

pub struct Player {
    pub server: ServerT,
    pub entity_id: u32,
    pub connection: WrappedConnection,
    pub disconnected: AtomicSet<bool>,
    pub username: AtomicSet<String>,
    pub uuid: AtomicSet<Uuid>,
    pub properties: AtomicSet<Vec<PlayerProperty>>,
    login_state: Mutex<Option<LoginState>>,
    pub level: AtomicRef<dyn LevelT + 'static>,
    pub world: AtomicRef<dyn WorldT + 'static>,
    pub entity: AtomicRef<RwLock<EntityT>>,
    pub data: RwLock<Option<IngamePlayerData>>,
    pub teleport_id: AtomicU32,
    pub latency: AtomicU64,
    pub last_ping: AtomicU64,
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

    pub async fn teleport_sync(self: &PlayerT) -> Result<()> {
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

    pub fn run_loops(self: &PlayerT) {
        tokio::spawn(self.clone().broadcast_event_loop(self.server.player_events.subscribe()));
        tokio::spawn(self.clone().ping_loop());
        tokio::spawn(self.clone().latency_loop());
    }

    pub async fn broadcast_event_loop(self: PlayerT, mut receiver: broadcast::Receiver<Arc<dyn PlayerEvent>>) {
        while let Ok(event) = receiver.recv().await {
            match event.execute(&self).await {
                Err(e) => {
                    warn!("player broadcast event failed for player {:?}: {:?}", *self.username, e);
                },
                _ => (),
            }
        }
        self.disconnect(ChatComponent::from("lagged out")).await.unwrap_or(());
    }

    pub async fn ping_loop(self: PlayerT) -> Result<()> {
        loop {
            let id = epoch();
            let last_ping = self.last_ping.load(Ordering::Relaxed);
            if last_ping > 0 && last_ping < id - 20000 {
                self.disconnect(ChatComponent::from("Timed out")).await?;
                return Ok(())
            }
            self.latency.store(id - last_ping, Ordering::Relaxed);
            self.send_packet(Packet::from(Game::from(game::clientbound::KeepAlivePacket {
                id: id as i64,
            }))).await?;
            delay_for(Duration::from_millis(5000)).await;
        }
    }

    pub async fn latency_loop(self: PlayerT) -> Result<()> {
        loop {
            self.server.send(PlayerLatencyEvent {
                uuid: (*self.uuid).clone(),
                latency: self.latency.load(Ordering::Relaxed),
            });
            delay_for(Duration::from_millis(60000)).await;
        }
    }

    async fn handle_game_packet(self: &PlayerT, packet: &GameServerbound) -> Result<()> {
        println!("{:?}", packet);
        match packet {
            GameServerbound::KeepAlivePacket(KeepAlivePacket { id }) => {
                self.last_ping.store(*id as u64, Ordering::Relaxed);
            },
            _ => (),
        }
        Ok(())
    }

}
