use std::sync::Mutex;

use rand::{seq::IteratorRandom, thread_rng};

use crate::{game::messages::Message, routes::error::Error};

use super::Class;

#[derive(Default, Debug)]
struct State {
    juliette: String,
}

#[derive(Default, Debug)]
pub struct Romeo {
    state: Mutex<State>,
}

impl Class for Romeo {
    fn init(
        &self,
        game_data: &crate::models::MergedGameData,
        player: &crate::game::player::Player,
    ) -> Result<(), crate::routes::error::Error> {
        let juliette = game_data
            .all_players
            .iter()
            .map(|p| &p.summoner_name)
            .filter(|p| &&player.name != p)
            .choose(&mut thread_rng())
            .ok_or_else(|| Error::Internal {
                msg: "Could not randomly choose juliette".into(),
            })?;

        self.state.lock().unwrap().juliette = juliette.clone();

        player.proxy.send_message(Message::Juliette {
            name: juliette.clone(),
        });

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
