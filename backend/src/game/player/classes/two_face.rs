use std::sync::Mutex;

use rand::{thread_rng, Rng};
use rand_distr::{Distribution, Uniform};
use serde::Serialize;

use crate::{
    game::messages::Message,
    models::{GameDataMutation, MergedGameDataMutation},
};

use super::Class;

#[derive(Default, Debug)]
struct State {
    inting: bool,
    next_swap_time: f64,
}

#[derive(Default, Debug)]
pub struct TwoFace {
    state: Mutex<State>,
}

impl Class for TwoFace {
    fn init(
        &self,
        _game_data: &crate::models::MergedGameData,
        player: &crate::game::player::Player,
    ) -> Result<(), crate::routes::error::Error> {
        let mut lock = self.state.lock().unwrap();

        lock.inting = thread_rng().gen();
        lock.next_swap_time = Uniform::new(2.0, 10.0).sample(&mut thread_rng());

        player.proxy.send_message(Message::TwoFaceState {
            inting: lock.inting,
        });

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

            if &lock.next_swap_time <= new_time {
                lock.inting = !lock.inting;
                lock.next_swap_time += Uniform::new(2.0, 10.0).sample(&mut thread_rng());

                player.proxy.send_message(Message::TwoFaceState {
                    inting: lock.inting,
                });
            }
        }

        Ok(())
    }

    fn state(&self) -> super::PlayerState {
        super::PlayerState::TwoFace(TwoFaceState {
            inting: self.state.lock().unwrap().inting,
        })
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub struct TwoFaceState {
    inting: bool,
}
