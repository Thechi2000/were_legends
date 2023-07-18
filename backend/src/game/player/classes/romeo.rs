use std::sync::Mutex;

use num_derive::FromPrimitive;
use rand::{seq::IteratorRandom, thread_rng};
use serde::Serialize;

use crate::{
    game::{messages::Message, GameInfo, GameInfoMutation},
    routes::error::Error,
};

use super::Class;

#[derive(Debug, FromPrimitive, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PlayerPosition {
    AllyTop = 0,
    AllyJungle,
    AllyMid,
    AllyBot,
    AllySupport,
    EnemyTop,
    EnemyJungle,
    EnemyMid,
    EnemyBot,
    EnemySupport,
}

const POSITIONS: [PlayerPosition; 10] = [
    PlayerPosition::AllyTop,
    PlayerPosition::AllyJungle,
    PlayerPosition::AllyMid,
    PlayerPosition::AllyBot,
    PlayerPosition::AllySupport,
    PlayerPosition::EnemyTop,
    PlayerPosition::EnemyJungle,
    PlayerPosition::EnemyMid,
    PlayerPosition::EnemyBot,
    PlayerPosition::EnemySupport,
];

#[derive(Debug, Clone, Serialize)]
pub struct Juliette {
    juliette: PlayerPosition,
    substitute: PlayerPosition,
}

#[derive(Default, Debug)]
struct State {
    juliette: Option<Juliette>,
}

#[derive(Default, Debug)]
pub struct Romeo {
    state: Mutex<State>,
}

impl Class for Romeo {
    fn init(
        &self,
        _game_data: &GameInfo,
        player: &crate::game::player::Player,
    ) -> Result<(), crate::routes::error::Error> {
        let juliette = *POSITIONS
            .iter()
            .choose(&mut thread_rng())
            .ok_or(Error::Internal {
                msg: "rand error".into(),
            })?;
        let juliette = Juliette {
            juliette,
            substitute: *POSITIONS
                .iter()
                .filter(|p| p != &&juliette)
                .choose(&mut thread_rng())
                .ok_or(Error::Internal {
                    msg: "rand error".into(),
                })?,
        };

        self.state.lock().unwrap().juliette = Some(juliette.clone());

        player.proxy.send_message(Message::Juliette {
            juliette: juliette.clone(),
        });

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
        super::PlayerState::Romeo(RomeoState {
            juliette: self.state.lock().unwrap().juliette.clone(),
        })
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub struct RomeoState {
    juliette: Option<Juliette>,
}
