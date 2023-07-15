use mutable::default_impl::VecMutation;
use serde::Serialize;

use crate::{
    game::team_builder::Role,
    models::{EventData, EventsMutation, MergedGameData, MergedGameDataMutation},
    routes::error::Error,
};

use self::{
    bot::{Bot, BotState},
    crook::{Crook, CrookState},
    impostor::{Impostor, ImpostorState},
    kamikaze::{Kamikaze, KamikazeState},
    romeo::{Romeo, RomeoState},
    super_hero::{SuperHero, SuperHeroState},
    two_face::{TwoFace, TwoFaceState},
};

use super::Player;

pub mod bot;
pub mod crook;
pub mod impostor;
pub mod kamikaze;
pub mod romeo;
pub mod super_hero;
pub mod two_face;

pub enum PlayerClass {
    SuperHero(SuperHero),
    Impostor(Impostor),
    Crook(Crook),
    Kamikaze(Kamikaze),
    Romeo(Romeo),
    TwoFace(TwoFace),
    Bot(Bot),
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "snake_case", tag = "class")]
pub enum PlayerState {
    SuperHero(SuperHeroState),
    Impostor(ImpostorState),
    Crook(CrookState),
    Kamikaze(KamikazeState),
    Romeo(RomeoState),
    TwoFace(TwoFaceState),
    Bot(BotState),
}

trait Class {
    fn init(
        &self,
        game_data: &crate::models::MergedGameData,
        player: &crate::game::player::Player,
    ) -> Result<(), Error>;
    fn update(
        &self,
        mutation: &MergedGameDataMutation,
        game_data: &MergedGameData,
        player: &Player,
    ) -> Result<(), Error>;
    fn state(&self) -> PlayerState;
}

impl PlayerClass {
    fn inner(&self) -> &dyn Class {
        match self {
            PlayerClass::Bot(i) => i,
            PlayerClass::SuperHero(i) => i,
            PlayerClass::Impostor(i) => i,
            PlayerClass::Crook(i) => i,
            PlayerClass::Kamikaze(i) => i,
            PlayerClass::Romeo(i) => i,
            PlayerClass::TwoFace(i) => i,
            
        }
    }

    pub fn receive_update(
        &self,
        mutation: &MergedGameDataMutation,
        game_data: &MergedGameData,
        player: &Player,
    ) -> Result<(), Error> {
        match mutation {
            MergedGameDataMutation::Events(EventsMutation::Events(VecMutation::Insertion(idx)))
                if game_data
                    .events
                    .events
                    .iter()
                    .find(|e| e.event_id == *idx)
                    .is_some_and(|e| matches!(e.data, EventData::GameStart)) =>
            {
                self.inner().init(game_data, player)
            }
            m => self.inner().update(m, game_data, player),
        }
    }

    pub fn get_state(&self) -> PlayerState {
        self.inner().state()
    }
}

impl From<Role> for PlayerClass {
    fn from(value: Role) -> Self {
        match value {
            Role::SuperHero => PlayerClass::SuperHero(Default::default()),
            Role::Impostor => PlayerClass::Impostor(Default::default()),
            Role::Crook => PlayerClass::Crook(Default::default()),
            Role::Kamikaze => PlayerClass::Kamikaze(Default::default()),
            Role::Romeo => PlayerClass::Romeo(Default::default()),
            Role::TwoFace => PlayerClass::TwoFace(Default::default()),
            Role::Bot => PlayerClass::Bot(Default::default()),
        }
    }
}
