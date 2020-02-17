use crate::result::*;
use async_trait::async_trait;
use basin2_protocol::network::*;
use basin2_protocol::packets::*;
use basin2_protocol::WrappedConnection;
use log::*;

pub struct Player {
    connection: WrappedConnection,
}

impl Player {
    pub async fn send_packet(&self, packet: Packet) -> Result<()> {
        let sent = {
            let mut sender = self.connection.outgoing.lock().unwrap();
            if sender.is_none() {
                return Err(basin_err!("connection already closing"));
            }
            sender.as_mut().unwrap().send(packet)
        };
        Ok(sent
            .await
            .map_err(|e| basin_err!("could not send packet: {:?}", e))?)
    }

    pub async fn disconnect(&self, reason: ChatComponent) -> Result<()> {
        let codec = self.connection.codec.0.read().unwrap();
        if codec.state == ConnectionProtocol::Login {
            self.send_packet(Packet::from(Login::from(LoginDisconnectPacket { reason })))
                .await?;
        } else if codec.state == ConnectionProtocol::Game {
            self.send_packet(Packet::from(Game::from(DisconnectPacket { reason })))
                .await?;
        }
        self.connection.disconnect();
        Ok(())
    }
}

#[async_trait]
pub trait ProtocolHandler {
    async fn handle_handshake(&self, packet: &HandshakeServerbound) -> Result<()>;
    async fn handle_login(&self, packet: &LoginServerbound) -> Result<()>;
    async fn handle_status(&self, packet: &StatusServerbound) -> Result<()>;
    async fn handle_game(&self, packet: &GameServerbound) -> Result<()>;
}

#[async_trait]
impl ProtocolHandler for Player {
    async fn handle_handshake(&self, packet: &HandshakeServerbound) -> Result<()> {
        use HandshakeServerbound::*;

        match packet {
            ClientIntentionPacket(packet) => {
                if basin2_protocol::PROTOCOL_VERSION != packet.protocolVersion as u32 {
                    return Err(basin_err!("invalid protocol version"));
                }
                if packet.intention == ConnectionProtocol::Status {
                    self.connection.set_state(ConnectionProtocol::Status);
                } else if packet.intention == ConnectionProtocol::Login {
                    self.connection.set_state(ConnectionProtocol::Login);
                } else {
                    return Err(basin_err!("invalid protocol version"));
                }
                Ok(())
            }
            _ => Err(basin_err!("invalid packet: {:?}", packet)),
        }
    }

    async fn handle_login(&self, packet: &LoginServerbound) -> Result<()> {
        Ok(())
    }

    async fn handle_status(&self, packet: &StatusServerbound) -> Result<()> {
        use StatusServerbound::*;

        match packet {
            StatusRequestPacket(packet) => Ok(()),
            PingRequestPacket(packet) => {
                self.send_packet(Packet::from(Status::from(PongResponsePacket {
                    time: packet.time,
                })))
                .await?;
                Ok(())
            }
            _ => Err(basin_err!("invalid packet: {:?}", packet)),
        }
    }

    async fn handle_game(&self, packet: &GameServerbound) -> Result<()> {
        Ok(())
    }
}

pub async fn handle_connection(connection: WrappedConnection) {
    // lock the mutex then claim the receiver for ourselves
    let mut incoming = connection.incoming.lock().unwrap().take().unwrap();
    let player = Player { connection };
    while let Some(packet) = incoming.recv().await {
        let result = match packet {
            Packet::Handshake(Handshake::HandshakeServerbound(packet)) => {
                player.handle_handshake(&packet)
            }
            Packet::Login(Login::LoginServerbound(packet)) => player.handle_login(&packet),
            Packet::Status(Status::StatusServerbound(packet)) => player.handle_status(&packet),
            Packet::Game(Game::GameServerbound(packet)) => player.handle_game(&packet),
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
            break;
        }
    }
    //TODO: close connection
}
