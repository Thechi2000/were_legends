use std::{sync::Mutex, time::Instant};

use rand::{thread_rng, Rng};
use rand_distr::{Distribution, Uniform};
use serde::Serialize;

use crate::{
    game::messages::Message,
    lol_api::spectator::{CurrentGameInfo, CurrentGameInfoMutation},
};

use super::Class;

#[derive(Default, Debug)]
struct State {
    inting: bool,
    next_swap_time: f64,
    game_start: Option<Instant>,
}

#[derive(Default, Debug)]
pub struct TwoFace {
    state: Mutex<State>,
}

impl Class for TwoFace {
    fn init(
        &self,
        _game_data: &CurrentGameInfo,
        player: &crate::game::player::Player,
    ) -> Result<(), crate::routes::error::Error> {
        let mut lock = self.state.lock().unwrap();

        lock.inting = thread_rng().gen();
        lock.next_swap_time = Uniform::new(5.0, 20.0).sample(&mut thread_rng());

        player.proxy.send_message(Message::TwoFaceState {
            inting: lock.inting,
        });

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
            .is_some_and(|start| lock.next_swap_time <= start.elapsed().as_secs_f64())
        {
            lock.inting = !lock.inting;
            lock.next_swap_time += Uniform::new(5.0, 20.0).sample(&mut thread_rng());

            println!(
                "Assigning new inting: {} at {:?}",
                lock.inting,
                lock.game_start.map(|s| s.elapsed().as_secs())
            );

            player.proxy.send_message(Message::TwoFaceState {
                inting: lock.inting,
            });
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
