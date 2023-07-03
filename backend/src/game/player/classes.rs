use mutable::Mutable;

use crate::models::MergedGameData;

use super::Player;

pub enum PlayerClass {}

impl PlayerClass {
    pub fn receive_update(
        &self,
        _mutation: &<Option<MergedGameData> as Mutable>::Mutation,
        _game_data: &Option<MergedGameData>,
        _player: &Player,
    ) {
        todo!()
    }
}
