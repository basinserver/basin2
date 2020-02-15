use super::connection::Connection;
use futures::stream::StreamExt;
use log::*;
use std::error::Error;
use tokio::net::TcpListener;

async fn start_server(address: &str) -> Result<(), Box<dyn Error>> {
    let mut listener = TcpListener::bind(address).await?;

    let server = async move {
        let mut incoming = listener.incoming();
        while let Some(socket_res) = incoming.next().await {
            match socket_res {
                Ok(socket) => {
                    println!("Accepted connection from: '{:?}'", socket.peer_addr());
                    tokio::spawn(Connection::spawn(socket));
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
