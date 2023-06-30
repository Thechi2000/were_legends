use self::{classes::PlayerClass, proxy::PlayerProxy};

pub mod classes;
pub mod proxy;

pub struct Player {
    class: Option<PlayerClass>,
    pub proxy: PlayerProxy,
}

impl Player {
    pub fn new(proxy: PlayerProxy) -> Self {
        Self {
            class: Default::default(),
            proxy,
        }
    }
}
