use crate::routes::error::Error;

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
    pub name: String,
}

impl Player {
    pub fn new(name: String, proxy: PlayerProxy) -> Self {
        Self {
            class: Default::default(),
            proxy,
            name,
        }
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
}
