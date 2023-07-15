use serde::Serialize;

use crate::{
    game::team_builder::Role,
    lol_api::spectator::{CurrentGameInfo, CurrentGameInfoMutation},
    routes::error::Error,
};

use self::{
    crook::{Crook, CrookState},
    droid::{Droid, DroidState},
    impostor::{Impostor, ImpostorState},
    kamikaze::{Kamikaze, KamikazeState},
    romeo::{Romeo, RomeoState},
    super_hero::{SuperHero, SuperHeroState},
    two_face::{TwoFace, TwoFaceState},
};

use super::Player;

pub mod crook;
pub mod droid;
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
    Droid(Droid),
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
    Droid(DroidState),
}

trait Class {
    fn init(
        &self,
        game_data: &CurrentGameInfo,
        player: &crate::game::player::Player,
    ) -> Result<(), Error>;
    fn update(
        &self,
        mutation: &CurrentGameInfoMutation,
        game_data: &CurrentGameInfo,
        player: &Player,
    ) -> Result<(), Error>;
    fn state(&self) -> PlayerState;
}

impl PlayerClass {
    fn inner(&self) -> &dyn Class {
        match self {
            PlayerClass::Droid(i) => i,
            PlayerClass::SuperHero(i) => i,
            PlayerClass::Impostor(i) => i,
            PlayerClass::Crook(i) => i,
            PlayerClass::Kamikaze(i) => i,
            PlayerClass::Romeo(i) => i,
            PlayerClass::TwoFace(i) => i,
        }
    }

    pub fn receive_mutation(
        &self,
        mutation: &CurrentGameInfoMutation,
        game_data: &CurrentGameInfo,
        player: &Player,
    ) -> Result<(), Error> {
        match mutation {
            CurrentGameInfoMutation::GameStartTime((0, _)) => self.inner().init(game_data, player),
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
            Role::Droid => PlayerClass::Droid(Default::default()),
        }
    }
}
