use self::player::{classes::PlayerState, proxy::PlayerProxy, Player};
use crate::{
    lol_api::{
        self,
        spectator::{CurrentGameInfo, CurrentGameInfoMutation},
    },
    routes::error::Error,
    session_management::UserSession,
};
use mutable::Mutable;
use serde::Serialize;
use std::{
    collections::{hash_map, HashMap},
    sync::{Arc, Weak},
    time::Duration,
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
    MatchDataMutation(Box<CurrentGameInfoMutation>),
    PlayerJoin { id: String, name: String },
}

/// Public status of a game
#[derive(Debug, Serialize)]
pub struct GameStatus {
    uid: Uuid,
    player_names: Vec<String>,
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
    data: RwLock<Option<CurrentGameInfo>>,
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
            player_names: self
                .players
                .values()
                .map(|p| p.session.name.clone())
                .collect(),
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
            player_names: self
                .players
                .values()
                .map(|p| p.session.name.clone())
                .collect(),
            has_started: self.data.read().await.is_some(),
            player_state: self.players.get(puuid).ok_or(Error::Unauthorized)?.state(),
        })
    }

    /// Returns whether the player with the given uuid is currently in this game
    pub fn has_player(&self, puuid: &String) -> bool {
        self.players.contains_key(puuid)
    }

    /// Returns the number of players in the game
    pub fn player_count(&self) -> usize {
        return self.players.len();
    }

    /// Add a player to this game
    ///
    /// - puuid: Puuid of the player to add
    /// - proxy: Proxy of the player to communicate messages
    pub async fn add_player(
        &mut self,
        session: UserSession,
        proxy: PlayerProxy,
    ) -> Result<(), Error> {
        if self.players.len() > 5 {
            Err(Error::MaxPlayerReached)
        } else if let hash_map::Entry::Vacant(e) = self.players.entry(session.puuid.clone()) {
            e.insert(Player::new(session.clone(), proxy));
            self.event_queue
                .send(GameEvent::PlayerJoin {
                    id: session.puuid,
                    name: session.name,
                })
                .await?;
            Ok(())
        } else {
            Err(Error::AlreadyInGame)
        }
    }

    /// Remove a player from this game
    ///
    /// - puuid: Puuid of the player to remove
    pub async fn remove_player(&mut self, puuid: String) -> Result<(), Error> {
        if let Some(_) = self.players.remove(&puuid) {
            Ok(())
        } else {
            Err(Error::NotInGame)
        }
    }

    /// Update the state of the game with data from the LoL Client API
    pub async fn update_state(&mut self, game_info: CurrentGameInfo) {
        let mut data = self.data.write().await;
        if data.is_none() {
            *data = Some(CurrentGameInfo::default());
        }

        let mutations = data.as_mut().unwrap().update(game_info);
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

    pub fn start(&mut self) -> Result<(), Error> {
        if self.player_count() != 5 {
            return Err(Error::NotEnoughPlayers);
        }

        let composition = team_builder::generate_composition();

        for (player, role) in self.players.values_mut().zip(composition.iter()) {
            player.set_role(*role)?;
        }

        Ok(())
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
                        if let Some(game_data) = state.read().await.data.read().await.as_ref() {
                            for player in state.read().await.players.values() {
                                if let Err(e) = player.receive_mutation(&m, game_data) {
                                    tracing::error!("Could not process game mutation: {:?}", e)
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    async fn fetch_updates(state: Weak<RwLock<Self>>) {
        let mut summoner_id = None;
        loop {
            tokio::time::sleep(Duration::from_secs(3)).await;

            if let Some(state) = state.upgrade() {
                if summoner_id.is_none() {
                    summoner_id = state
                        .read()
                        .await
                        .players
                        .values()
                        .next()
                        .map(|p| p.session.summoner_id.clone());
                }

                if let Some(ref summoner_id) = summoner_id {
                    if let Ok(match_info) =
                        lol_api::spectator::get_active_game(summoner_id.clone()).await
                    {
                        state.write().await.update_state(match_info).await;
                    }
                }
            } else {
                break;
            }
        }
    }
}
