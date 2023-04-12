use mutable::Mutable;

use self::classes::PlayerClass;

use super::MergedAllGameData;

pub mod classes;
pub mod proxy;

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
