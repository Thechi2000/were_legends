use std::sync::Mutex;

use num_derive::FromPrimitive;
use num_traits::FromPrimitive;
use rand::{thread_rng, Rng};
use rand_distr::{Distribution, Normal};
use serde::Serialize;

use crate::{
    game::{messages::Message, GameInfo, GameInfoMutation},
    routes::error::Error,
};

use super::{Class, PlayerState};

#[derive(Debug, Clone, Serialize, FromPrimitive)]
#[serde(rename_all = "snake_case")]
pub enum Mission {
    Summoners = 0,
    GoTop,
    GoBot,
    TakeBlue,
    TakeRed,
    Sing,
    ExhortTeam,
    StayBase,
    IntPingMs,
    Emote,
    QOnCd,
    WOnCd,
    EOnCd,
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
        _game_data: &GameInfo,
        _player: &crate::game::player::Player,
    ) -> Result<(), crate::routes::error::Error> {
        let mut lock = self.state.lock().unwrap();

        lock.next_mission_timestamp = Normal::new(600.0, 120.0)
            .map_err(Error::from)?
            .sample(&mut thread_rng());

        Ok(())
    }

    fn update(
        &self,
        mutation: &GameInfoMutation,
        _game_data: &GameInfo,
        player: &crate::game::player::Player,
    ) -> Result<(), crate::routes::error::Error> {
        let mut lock = self.state.lock().unwrap();

        #[allow(irrefutable_let_patterns)]
        if let GameInfoMutation::Duration((_, new_time)) = mutation {
            if lock.next_mission_timestamp <= *new_time as f64 {
                lock.next_mission_timestamp += Normal::new(300.0, 120.0)
                    .map_err(Error::from)?
                    .sample(&mut thread_rng());

                lock.mission =
                    <Mission as FromPrimitive>::from_usize(thread_rng().gen_range(0..12));

                println!(
                    "Assigning new mission: {:?} at {}",
                    lock.mission, new_time
                );

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
