use crate::game::{messages::Message, GameState};
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};
use uuid::Uuid;

pub struct State {
    pub games: Vec<GameState>,
    pub messages: HashMap<Uuid, Arc<Mutex<Vec<Message>>>>,
}

impl State {
    pub fn new() -> Self {
        Self {
            games: vec![],
            messages: HashMap::default(),
        }
    }

    pub fn get_game_by_player(&self, uuid: Uuid) -> Option<&GameState> {
        self.games.iter().find(|g| g.has_player(uuid))
    }
}

impl Default for State {
    fn default() -> Self {
        Self::new()
    }
}
