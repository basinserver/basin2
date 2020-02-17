use super::*;
use crate::result::*;
use crate::player::*;

struct PlayerJoin {
    player: PlayerT,
}

impl ServerEvent for PlayerJoin {
    fn process(self, server: ServerT, player: PlayerT) {
        
    }
}