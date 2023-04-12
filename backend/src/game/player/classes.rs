use mutable::Mutable;

use crate::game::MergedAllGameData;

use super::Player;

pub enum PlayerClass {}

impl PlayerClass {
    pub fn receive_update(
        &self,
        _mutation: &<Option<MergedAllGameData> as Mutable>::Mutation,
        _game_data: &Option<MergedAllGameData>,
        _player: &Player,
    ) {
        todo!()
    }
}
