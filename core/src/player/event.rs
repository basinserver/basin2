use super::*;

#[async_trait]
pub trait PlayerEvent: Send + Sync + 'static {
    async fn execute(&self, player: &PlayerT) -> Result<()>;
}

pub struct PlayerJoinEvent {
    pub from: PlayerT,
}

#[async_trait]
impl PlayerEvent for PlayerJoinEvent {
    async fn execute(&self, player: &PlayerT) -> Result<()> {
        let game_type = self.from.data.read().unwrap().as_ref().unwrap().game_type;
        player.send_packet(Packet::from(Game::from(PlayerInfoPacket {
            action: PlayerInfoPacketAction::AddPlayer,
            data: vec![(
                *self.from.uuid,
                PlayerInfoPacketData::AddPlayer(
                    (*self.from.username).clone(),
                    (*self.from.properties).clone(),
                    game_type,
                    0,
                    None,
                )
            )],
        }))).await?;
        Ok(())
    }
}

pub struct PlayerLatencyEvent {
    pub uuid: Uuid,
    pub latency: u64,
}

#[async_trait]
impl PlayerEvent for PlayerLatencyEvent {
    async fn execute(&self, player: &PlayerT) -> Result<()> {
        player.send_packet(Packet::from(Game::from(PlayerInfoPacket {
            action: PlayerInfoPacketAction::UpdateLatency,
            data: vec![(
                self.uuid.clone(),
                PlayerInfoPacketData::UpdateLatency(
                    self.latency as i32,
                )
            )],
        }))).await?;
        Ok(())
    }
}

pub struct ChatBroadcastEvent {
    pub component: ChatComponent,
    pub chat_type: ChatType,
}

#[async_trait]
impl PlayerEvent for ChatBroadcastEvent {
    async fn execute(&self, player: &PlayerT) -> Result<()> {
        player.send_packet(Packet::from(Game::from(game::clientbound::ChatPacket {
            chat_type: self.chat_type,
            message: self.component.clone(),
        }))).await?;
        Ok(())
    }
}