use serde::Serialize;

use super::Class;

#[derive(Default, Debug)]
pub struct SuperHero {}
impl Class for SuperHero {
    fn init(
        &self,
        _game_data: &crate::models::MergedGameData,
        _player: &crate::game::player::Player,
    ) -> Result<(), crate::routes::error::Error> {
        Ok(())
    }

    fn update(
        &self,
        _mutation: &crate::models::MergedGameDataMutation,
        _game_data: &crate::models::MergedGameData,
        _player: &crate::game::player::Player,
    ) -> Result<(), crate::routes::error::Error> {
        Ok(())
    }

    fn state(&self) -> super::PlayerState {
        super::PlayerState::SuperHero(SuperHeroState)
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub struct SuperHeroState;
