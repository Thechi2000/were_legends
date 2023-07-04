use mutable::Mutable;

use crate::{models::MergedGameData, game::team_builder::Role};

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

impl From<Role> for PlayerClass {
    fn from(value: Role) -> Self {
        todo!()
    }
}