use super::MergedAllGameData;
use mutable::Mutable;

pub enum PlayerClass {}
pub struct PlayerProxy {
    // TODO
}

pub struct Player {
    class: PlayerClass,
}

impl Player {
    pub fn receive_update(
        &self,
        mutation: &<Option<MergedAllGameData> as Mutable>::Mutation,
        game_data: &Option<MergedAllGameData>,
    ) {
        self.class.receive_update(mutation, game_data, self)
    }
}


impl PlayerClass {
    pub fn receive_update(
        &self,
        mutation: &<Option<MergedAllGameData> as Mutable>::Mutation,
        game_data: &Option<MergedAllGameData>,
        player: &Player,
    ) {
        todo!()
    }
}