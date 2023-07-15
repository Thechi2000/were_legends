use std::sync::Mutex;

use rand::{thread_rng, Rng};
use rand_derive::Rand;
use rand_distr::{Distribution, Normal};
use serde::Serialize;

use crate::{
    game::messages::Message,
    lol_api::spectator::{CurrentGameInfo, CurrentGameInfoMutation},
    routes::error::Error,
};

use super::{Class, PlayerState};

#[derive(Debug, Rand, Clone, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Mission {
    SumBeforeTwo,
}

#[derive(Default, Debug)]
struct State {
    mission: Option<Mission>,
    next_mission_timestamp: f64,
}

#[derive(Default, Debug)]
pub struct Droid {
    state: Mutex<State>,
}

impl Class for Droid {
    fn init(
        &self,
        _game_data: &CurrentGameInfo,
        _player: &crate::game::player::Player,
    ) -> Result<(), crate::routes::error::Error> {
        let mut lock = self.state.lock().unwrap();

        lock.next_mission_timestamp = Normal::new(120.0, 30.0)
            .map_err(Error::from)?
            .sample(&mut thread_rng());

        Ok(())
    }

    fn update(
        &self,
        mutation: &CurrentGameInfoMutation,
        _game_data: &CurrentGameInfo,
        player: &crate::game::player::Player,
    ) -> Result<(), crate::routes::error::Error> {
        if let CurrentGameInfoMutation::GameLength((_, new_time)) = mutation {
            let mut lock = self.state.lock().unwrap();
            if lock.next_mission_timestamp <= *new_time as f64 {
                lock.next_mission_timestamp += Normal::new(300.0, 60.0)
                    .map_err(Error::from)?
                    .sample(&mut thread_rng());
                lock.mission = Some(thread_rng().gen());

                player.proxy.send_message(Message::Mission {
                    mission: lock.mission.clone().unwrap(),
                })
            }
        }

        Ok(())
    }

    fn state(&self) -> super::PlayerState {
        PlayerState::Droid(DroidState {
            mission: self.state.lock().unwrap().mission.clone(),
        })
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub struct DroidState {
    mission: Option<Mission>,
}
