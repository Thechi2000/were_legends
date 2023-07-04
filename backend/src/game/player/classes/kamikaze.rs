use super::Class;

#[derive(Default, Debug)]
pub struct Kamikaze {}
impl Class for Kamikaze {
    fn init(&self,
        _game_data: &crate::models::MergedGameData,
        _player: &crate::game::player::Player,) -> Result<(), crate::routes::error::Error> {
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
}
