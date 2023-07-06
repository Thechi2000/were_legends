use crate::{
    game::{player::proxy::PlayerProxy, GameState},
};
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};
use tokio::sync::RwLock;
use uuid::Uuid;

/// Global state of the app
pub struct State {
    /// All running games
    pub games: HashMap<Uuid, Arc<RwLock<GameState>>>,
    /// Proxies for each logged in player
    pub messages: Mutex<HashMap<String, PlayerProxy>>,
}

impl State {
    pub fn new() -> Self {
        Self {
            games: Default::default(),
            messages: Default::default(),
        }
    }

    /// Creates a new game and returns its uuid and a handle to acces it
    pub fn create_game(&mut self) -> (Uuid, Arc<RwLock<GameState>>) {
        let uid = Uuid::new_v4();
        self.games.insert(uid, GameState::new(uid));
        (uid, self.get_game_by_id(uid).unwrap())
    }

    /// Returns the game in which the player is currently playing, if there is one
    pub async fn get_game_by_player(
        &self,
        name: &String,
    ) -> Option<(Uuid, Arc<RwLock<GameState>>)> {
        for g in self.games.iter() {
            if g.1.read().await.has_player(name) {
                return Some((*g.0, g.1.clone()));
            }
        }

        None
    }

    /// Returns the game with the corresponding uuid
    pub fn get_game_by_id(&self, uuid: Uuid) -> Option<Arc<RwLock<GameState>>> {
        self.games.get(&uuid).cloned()
    }

    /// Get the proxy for a player, or creates it if missing
    pub fn get_or_create_proxy(&self, name: &String) -> PlayerProxy {
        let mut lock = self.messages.lock().unwrap();

        if let std::collections::hash_map::Entry::Vacant(e) = lock.entry(name.clone()) {
            let proxy = PlayerProxy::default();
            e.insert(proxy.clone());
            proxy
        } else {
            lock.get(name).unwrap().clone()
        }
    }
}

impl Default for State {
    fn default() -> Self {
        Self::new()
    }
}
