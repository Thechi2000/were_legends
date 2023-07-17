use std::{sync::Mutex, time::Instant};

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
    game_start: Option<Instant>,
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

        lock.next_mission_timestamp = Normal::new(20.0, 2.0)
            .map_err(Error::from)?
            .sample(&mut thread_rng());

        Ok(())
    }

    fn update(
        &self,
        _mutation: &CurrentGameInfoMutation,
        game_data: &CurrentGameInfo,
        player: &crate::game::player::Player,
    ) -> Result<(), crate::routes::error::Error> {
        let mut lock = self.state.lock().unwrap();
        if game_data.game_start_time != 0 && lock.game_start.is_none() {
            lock.game_start = Some(Instant::now());
        }

        if lock
            .game_start
            .is_some_and(|start| lock.next_mission_timestamp <= start.elapsed().as_secs_f64())
        {
            lock.next_mission_timestamp += Normal::new(20.0, 2.0)
                .map_err(Error::from)?
                .sample(&mut thread_rng());
            lock.mission = Some(thread_rng().gen());

            println!(
                "Assigning new mission: {:?} at {:?}",
                lock.mission,
                lock.game_start.map(|s| s.elapsed().as_secs())
            );

            player.proxy.send_message(Message::Mission {
                mission: lock.mission.clone().unwrap(),
            })
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
