use crate::{
    game::{player::proxy::PlayerProxy, GameState},
    lol_api::summoners::Puuid,
};
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};
use tokio::sync::RwLock;
use uuid::Uuid;

pub struct State {
    pub games: HashMap<Uuid, Arc<RwLock<GameState>>>,
    pub messages: Mutex<HashMap<Puuid, PlayerProxy>>,
}

impl State {
    pub fn new() -> Self {
        Self {
            games: Default::default(),
            messages: Default::default(),
        }
    }

    pub fn create_game(&mut self) -> (Uuid, Arc<RwLock<GameState>>) {
        let uid = Uuid::new_v4();
        self.games.insert(uid, GameState::new(uid));
        (uid, self.get_game_by_id(uid).unwrap())
    }

    pub async fn get_game_by_player(
        &self,
        puuid: &Puuid,
    ) -> Option<(Uuid, Arc<RwLock<GameState>>)> {
        for g in self.games.iter() {
            if g.1.read().await.has_player(puuid) {
                return Some((*g.0, g.1.clone()));
            }
        }

        None
    }

    pub fn get_game_by_id(&self, uuid: Uuid) -> Option<Arc<RwLock<GameState>>> {
        self.games.get(&uuid).cloned()
    }

    pub fn get_or_create_proxy(&self, puuid: &Puuid) -> PlayerProxy {
        let mut lock = self.messages.lock().unwrap();

        if let std::collections::hash_map::Entry::Vacant(e) = lock.entry(puuid.clone()) {
            let proxy = PlayerProxy::default();
            e.insert(proxy.clone());
            proxy
        } else {
            lock.get(puuid).unwrap().clone()
        }
    }
}

impl Default for State {
    fn default() -> Self {
        Self::new()
    }
}
