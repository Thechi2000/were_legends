use self::player::Player;
use crate::models::{ActivePlayer, Events, GameData, PlayerData};
use mutable::{cmp::SoftEq, Mutable};
use std::{collections::HashMap, sync::RwLock};
use uuid::Uuid;

pub mod messages;
pub mod player;

pub struct GameState {
    players: HashMap<Uuid, Player>,
    data: RwLock<Option<MergedAllGameData>>,
}

#[derive(Mutable, Clone, Debug, PartialEq)]
pub struct MergedAllGameData {
    pub all_players: Vec<MergedPlayerData>,
    pub events: Events,
    pub game_data: GameData,
}

#[derive(Mutable, Clone, Debug, PartialEq)]
pub struct MergedPlayerData {
    pub active: Option<ActivePlayer>,
    pub player: PlayerData,
}

impl SoftEq for MergedPlayerData {
    type Uid = String;

    fn uid(&self) -> Self::Uid {
        self.player.summoner_name.clone()
    }
}

impl GameState {
    pub fn new() -> Self {
        Self {
            players: HashMap::new(),
            data: RwLock::new(None),
        }
    }

    pub fn has_player(&self, uuid: Uuid) -> bool {
        self.players.contains_key(&uuid)
    }

    pub fn update_state(&self, new_state: Option<MergedAllGameData>) {
        let mutations = <Option<MergedAllGameData> as Mutable>::update(
            self.data.write().as_mut().unwrap(),
            new_state,
        );
        for mutation in mutations.iter() {
            for player in self.players.values() {
                player.receive_update(mutation, &self.data.read().unwrap())
            }
        }
    }
}

impl Default for GameState {
    fn default() -> Self {
        Self::new()
    }
}
