use crate::{
    lol_api::spectator::{CurrentGameInfo, CurrentGameInfoMutation},
    routes::error::Error,
    session_management::UserSession,
};

use self::{
    classes::{PlayerClass, PlayerState},
    proxy::PlayerProxy,
};

use super::team_builder::Role;

pub mod classes;
pub mod proxy;

pub struct Player {
    class: Option<PlayerClass>,
    pub proxy: PlayerProxy,
    pub session: UserSession,
}

impl Player {
    pub fn new(session: UserSession, proxy: PlayerProxy) -> Self {
        Self {
            class: Default::default(),
            proxy,
            session,
        }
    }

    pub fn receive_mutation(
        &self,
        mutation: &CurrentGameInfoMutation,
        game_data: &CurrentGameInfo,
    ) -> Result<(), Error> {
        if let Some(ref class) = self.class {
            class.receive_mutation(&mutation, game_data, self)?;
        }

        Ok(())
    }

    pub fn state(&self) -> Option<PlayerState> {
        self.class.as_ref().map(|c| c.get_state())
    }

    pub fn set_role(&mut self, role: Role) -> Result<(), Error> {
        if self.class.is_some() {
            return Err(Error::AlreadyStarted);
        }

        self.class = Some(PlayerClass::from(role));
        self.proxy
            .send_message(super::messages::Message::Role { role: role });

        Ok(())
    }

    pub fn role(&self) -> Option<Role> {
        self.class.as_ref().map(|p| match p {
            PlayerClass::SuperHero(_) => Role::SuperHero,
            PlayerClass::Impostor(_) => Role::Impostor,
            PlayerClass::Crook(_) => Role::Crook,
            PlayerClass::Kamikaze(_) => Role::Kamikaze,
            PlayerClass::Romeo(_) => Role::Romeo,
            PlayerClass::TwoFace(_) => Role::TwoFace,
            PlayerClass::Droid(_) => Role::Droid,
        })
    }
}
