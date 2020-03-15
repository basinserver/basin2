use crate::player::handle_connection;
use basin2_lib::result::*;
use crate::util::CONFIG;
use basin2_protocol::{start_server, WrappedConnection};
use tokio::sync::mpsc;
use crate::world::Level;
use crate::player::PlayerT;
use uuid::Uuid;
use std::collections::HashMap;
use std::sync::{ Arc, Weak, RwLock };
use tokio::sync::broadcast;
pub mod events;
use basin2_protocol::network::{ CommandNode, BaseCommandNode };
use linked_hash_map::LinkedHashMap;

pub trait ServerEvent: Send + Sync {
    fn process(self, server: ServerT, player: PlayerT);
}

pub struct Server {
    pub level: Level,
    pub players: RwLock<HashMap<Uuid, Weak<PlayerT>>>,
    pub events: broadcast::Sender<Box<dyn ServerEvent>>,
    pub commands: Arc<CommandNode>,
}

impl Server {
    pub fn new(level: Level) -> Server {
        let mut commands = CommandNode::Root {
            node: RwLock::new(BaseCommandNode {
                uuid: Uuid::new_v4(),
                children: LinkedHashMap::new(),
                redirect: None,
                command: false,
            })
        };
        crate::command::register_commands(&mut commands);
        Server {
            level,
            players: RwLock::new(HashMap::new()),
            events: broadcast::channel(1024).0,
            commands: Arc::new(commands),
        }
    }
}

pub type ServerT = Arc<Server>;

pub async fn start(server: ServerT) {
    let (sender, receiver) = mpsc::channel(256);
    tokio::spawn(start_server(
        format!("{}:{}", CONFIG.bind_address, CONFIG.bind_port),
        sender,
    ));
    handle_connections(receiver, server).await;
}

async fn handle_connections(mut receiver: mpsc::Receiver<WrappedConnection>, server: ServerT) {
    while let Some(connection) = receiver.recv().await {
        tokio::spawn(handle_connection(connection, server.clone()));
    }
    panic!("connections are no longer being accepted!");
}
