use std::sync::Mutex;

use rand::{seq::IteratorRandom, thread_rng};
use serde::Serialize;

use crate::{
    game::messages::Message,
    lol_api::spectator::{CurrentGameInfo, CurrentGameInfoMutation},
    routes::error::Error,
};

use super::Class;

#[derive(Default, Debug)]
struct State {
    juliette: Option<String>,
}

#[derive(Default, Debug)]
pub struct Romeo {
    state: Mutex<State>,
}

impl Class for Romeo {
    fn init(
        &self,
        game_data: &CurrentGameInfo,
        player: &crate::game::player::Player,
    ) -> Result<(), crate::routes::error::Error> {
        let juliette = game_data
            .participants
            .iter()
            .map(|p| &p.summoner_name)
            .filter(|p| &&player.session.name != p)
            .choose(&mut thread_rng())
            .ok_or_else(|| Error::Internal {
                msg: "Could not randomly choose juliette".into(),
            })?;

        self.state.lock().unwrap().juliette = Some(juliette.clone());

        player.proxy.send_message(Message::Juliette {
            name: juliette.clone(),
        });

        Ok(())
    }

    fn update(
        &self,
        _mutation: &CurrentGameInfoMutation,
        _game_data: &CurrentGameInfo,
        _player: &crate::game::player::Player,
    ) -> Result<(), crate::routes::error::Error> {
        Ok(())
    }

    fn state(&self) -> super::PlayerState {
        super::PlayerState::Romeo(RomeoState {
            juliette: self.state.lock().unwrap().juliette.clone(),
        })
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub struct RomeoState {
    juliette: Option<String>,
}
