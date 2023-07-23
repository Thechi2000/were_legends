use std::sync::Mutex;

use rand::{thread_rng, Rng};
use rand_distr::{Distribution, Uniform};
use serde::Serialize;

use crate::game::{messages::Message, GameInfo, GameInfoMutation};

use super::Class;

#[derive(Default, Debug, Clone)]
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
        _game_data: &GameInfo,
        player: &crate::game::player::Player,
    ) -> Result<(), crate::routes::error::Error> {
        let mut lock = self.state.lock().unwrap();

        lock.inting = thread_rng().gen();
        lock.next_swap_time = Uniform::new(300.0, 600.0).sample(&mut thread_rng());

        player.proxy.send_message(Message::TwoFaceState {
            inting: lock.inting,
        });

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
            if lock.next_swap_time <= *new_time as f64 {
                lock.inting = !lock.inting;
                lock.next_swap_time += Uniform::new(300.0, 600.0).sample(&mut thread_rng());

                println!("Assigning new inting: {} at {:?}", lock.inting, new_time);

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
