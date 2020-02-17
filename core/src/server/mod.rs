use crate::player::handle_connection;
use crate::result::*;
use crate::util::CONFIG;
use basin2_protocol::{start_server, WrappedConnection};
use tokio::sync::mpsc;

pub fn start() {
    let (sender, receiver) = mpsc::channel(256);

    tokio::spawn(start_server(
        format!("{}:{}", CONFIG.bind_address, CONFIG.bind_port),
        sender,
    ));
    tokio::spawn(handle_connections(receiver));
}

async fn handle_connections(mut receiver: mpsc::Receiver<WrappedConnection>) {
    while let Some(connection) = receiver.recv().await {
        tokio::spawn(handle_connection(connection));
    }
    panic!("connections are no longer being accepted!");
}
