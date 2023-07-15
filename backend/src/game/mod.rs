use self::player::{classes::PlayerState, proxy::PlayerProxy, Player};
use crate::{
    lol_api::{self},
    models::{AllGameData, MergedGameData, MergedGameDataMutation},
    routes::error::Error,
};
use mutable::Mutable;
use serde::Serialize;
use std::{
    collections::{hash_map, HashMap},
    sync::Arc,
};
use tokio::sync::{
    mpsc::{Receiver, Sender},
    RwLock,
};
use uuid::Uuid;

pub mod messages;
pub mod player;
pub mod team_builder;

pub enum GameEvent {
    MatchDataMutation(Box<MergedGameDataMutation>),
    PlayerJoin { id: String, name: String },
    GameStart,
}

/// Public status of a game
#[derive(Debug, Serialize)]
pub struct GameStatus {
    uid: Uuid,
    player_names: Vec<String>,
    has_started: bool,
}

/// Public status of a game, augmented with the state of the player
#[derive(Debug, Serialize)]
pub struct AuthenticatedGameStatus {
    uid: Uuid,
    player_names: Vec<String>,
    has_started: bool,
    player_state: Option<PlayerState>,
}

/// State of a game
pub struct GameState {
    uid: Uuid,
    players: HashMap<String, Player>,
    data: RwLock<Option<MergedGameData>>,
    event_queue: Sender<GameEvent>,
}

impl GameState {
    pub fn new(uid: Uuid) -> Arc<RwLock<Self>> {
        let (tx, rx) = tokio::sync::mpsc::channel(100);
        let state = Arc::new(RwLock::new(Self {
            uid,
            players: Default::default(),
            data: Default::default(),
            event_queue: tx,
        }));

        tokio::spawn(Self::listen_events(rx, state.clone()));

        state
    }

    /// Returns the public status of the game
    pub async fn get_status(&self) -> GameStatus {
        GameStatus {
            uid: self.uid,
            player_names: self.players.values().map(|p| p.name.clone()).collect(),
            has_started: self.data.read().await.is_some(),
        }
    }

    /// Returns the public status of the game, along with the status of one player
    ///
    /// - puuid: Puuid of the player of which the status is requested
    pub async fn get_status_authenticated(
        &self,
        puuid: &String,
    ) -> Result<AuthenticatedGameStatus, Error> {
        Ok(AuthenticatedGameStatus {
            uid: self.uid,
            player_names: self.players.values().map(|p| p.name.clone()).collect(),
            has_started: self.data.read().await.is_some(),
            player_state: self.players.get(puuid).ok_or(Error::Unauthorized)?.state(),
        })
    }

    /// Returns whether the player with the given uuid is currently in this game
    pub fn has_player(&self, puuid: &String) -> bool {
        self.players.contains_key(puuid)
    }

    /// Add a player to this game
    ///
    /// - puuid: Puuid of the player to add
    /// - proxy: Proxy of the player to communicate messages
    pub async fn add_player(&mut self, puuid: String, proxy: PlayerProxy) -> Result<(), Error> {
        let player_name = lol_api::account::get_by_puuid(puuid.clone())
            .await?
            .game_name;

        if self.players.len() > 5 {
            Err(Error::MaxPlayerReached)
        } else if let hash_map::Entry::Vacant(e) = self.players.entry(puuid.clone()) {
            e.insert(Player::new(player_name.clone(), proxy));
            self.event_queue
                .send(GameEvent::PlayerJoin {
                    id: puuid,
                    name: player_name,
                })
                .await?;
            Ok(())
        } else {
            Err(Error::AlreadyInGame)
        }
    }

    /// Update the state of the game with data from the LoL Client API
    pub async fn update_state(&mut self, data: AllGameData) {
        let merged_game_data = MergedGameData::from(data);

        let mut data = self.data.write().await;
        if data.is_none() {
            *data = Some(MergedGameData::default());
        }

        let mutations = data.as_mut().unwrap().update(merged_game_data);
        for mutation in mutations {
            if let Err(e) = self
                .event_queue
                .send(GameEvent::MatchDataMutation(Box::new(mutation)))
                .await
            {
                tracing::error!("Could not send mutation to event queue: {e}")
            }
        }
    }

    /// Background task listening and processing events
    async fn listen_events(mut rx: Receiver<GameEvent>, state: Arc<RwLock<Self>>) {
        loop {
            while let Some(event) = rx.recv().await {
                match event {
                    GameEvent::PlayerJoin { name, .. } => {
                        for player in state.read().await.players.values() {
                            player
                                .proxy
                                .send_message(messages::Message::PlayerJoin { name: name.clone() })
                        }
                    }
                    GameEvent::MatchDataMutation(m) => {
                        for player in state.read().await.players.values() {
                            player.proxy.send_message(messages::Message::Debug {
                                value: format!("{m:#?}"),
                            })
                        }
                    }
                    _ => todo!(),
                }
            }
        }
    }
}
