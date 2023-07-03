use self::{classes::PlayerClass, proxy::PlayerProxy};

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
}
