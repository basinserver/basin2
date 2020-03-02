use super::*;
use basin2_lib::result::*;
use crate::player::*;

struct PlayerJoin {
    player: PlayerT,
}

impl ServerEvent for PlayerJoin {
    fn process(self, server: ServerT, player: PlayerT) {
        
    }
}