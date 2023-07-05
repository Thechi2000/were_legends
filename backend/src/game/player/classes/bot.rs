use std::sync::Mutex;

use rand::{thread_rng, Rng};
use rand_derive::Rand;
use rand_distr::{Distribution, Normal};
use serde::Serialize;

use crate::{
    game::messages::Message,
    models::{GameDataMutation, MergedGameDataMutation},
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
pub struct Bot {
    state: Mutex<State>,
}

impl Class for Bot {
    fn init(
        &self,
        _game_data: &crate::models::MergedGameData,
        _player: &crate::game::player::Player,
    ) -> Result<(), crate::routes::error::Error> {
        let mut lock = self.state.lock().unwrap();

        lock.next_mission_timestamp = Normal::new(2.0, 0.5)
            .map_err(Error::from)?
            .sample(&mut thread_rng());

        Ok(())
    }

    fn update(
        &self,
        mutation: &crate::models::MergedGameDataMutation,
        _game_data: &crate::models::MergedGameData,
        player: &crate::game::player::Player,
    ) -> Result<(), crate::routes::error::Error> {
        if let MergedGameDataMutation::GameData(GameDataMutation::GameTime((_, new_time))) =
            mutation
        {
            let mut lock = self.state.lock().unwrap();
            if lock.next_mission_timestamp <= *new_time {
                lock.next_mission_timestamp += Normal::new(5.0, 1.0)
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
        PlayerState::Bot(BotState { mission: self.state.lock().unwrap().mission.clone() })
    }   
}

#[derive(Debug, Serialize)]
#[serde(rename_all="snake_case")]
pub struct BotState {
    mission: Option<Mission>
}
