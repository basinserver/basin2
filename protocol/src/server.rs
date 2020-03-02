use super::connection::Connection;
use crate::connection::WrappedConnection;
use basin2_lib::result::*;
use futures::stream::StreamExt;
use log::*;
use tokio::net::TcpListener;
use tokio::sync::mpsc;

pub async fn start_server(address: String, handler: mpsc::Sender<WrappedConnection>) -> Result<()> {
    let mut listener = TcpListener::bind(address.clone()).await?;

    let server = async move {
        let mut incoming = listener.incoming();
        while let Some(socket_res) = incoming.next().await {
            match socket_res {
                Ok(socket) => {
                    info!("Accepted connection from: '{:?}'", socket.peer_addr());
                    tokio::spawn(Connection::spawn(socket, true, handler.clone()));
                }
                Err(err) => {
                    warn!("Error accepting connection: '{:?}'", err);
                }
            }
        }
    };

    info!("Listening on {}", address);

    server.await;
    Ok(())
}
