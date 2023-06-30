use mutable::Mutable;

use crate::models::MatchDto;

use super::Player;

pub enum PlayerClass {}

impl PlayerClass {
    pub fn receive_update(
        &self,
        _mutation: &<Option<MatchDto> as Mutable>::Mutation,
        _game_data: &Option<MatchDto>,
        _player: &Player,
    ) {
        todo!()
    }
}
