use serde::Serialize;


use crate::game::{GameInfo, GameInfoMutation};

use super::Class;

#[derive(Default, Debug)]
pub struct Crook {}
impl Class for Crook {
    fn init(
        &self,
        _game_data: &GameInfo,
        _player: &crate::game::player::Player,
    ) -> Result<(), crate::routes::error::Error> {
        Ok(())
    }

    fn update(
        &self,
        _mutation: &GameInfoMutation,
        _game_data: &GameInfo,
        _player: &crate::game::player::Player,
    ) -> Result<(), crate::routes::error::Error> {
        Ok(())
    }

    fn state(&self) -> super::PlayerState {
        super::PlayerState::Crook(CrookState)
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub struct CrookState;
