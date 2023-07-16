use self::{
    messages::Message,
    player::{classes::PlayerState, proxy::PlayerProxy, Player},
    team_builder::Role,
};
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
    #[serde(skip_serializing_if = "Option::is_none")]
    votes: Option<HashMap<String, HashMap<String, Role>>>,
    state: State,
    roles: Option<HashMap<String, Role>>,
}

/// Public status of a game, augmented with the state of the player
#[derive(Debug, Serialize)]
pub struct AuthenticatedGameStatus {
    uid: Uuid,
    player_names: Vec<String>,
    has_started: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    player_state: Option<PlayerState>,
    #[serde(skip_serializing_if = "Option::is_none")]
    votes: Option<HashMap<String, HashMap<String, Role>>>,
    state: State,
    roles: Option<HashMap<String, Role>>,
}

#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "snake_case", tag = "state")]
pub enum State {
    NotStarted,
    InGame,
    WaitingVotes { players: Vec<String> },
    Finished,
}

/// State of a game
pub struct GameState {
    uid: Uuid,
    players: HashMap<String, Player>,
    data: RwLock<Option<CurrentGameInfo>>,
    event_queue: Sender<GameEvent>,
    votes: HashMap<String, HashMap<String, Role>>,
    state: State,
}

impl GameState {
    pub fn new(uid: Uuid) -> Arc<RwLock<Self>> {
        let (tx, rx) = tokio::sync::mpsc::channel(100);
        let state = Arc::new(RwLock::new(Self {
            uid,
            players: Default::default(),
            data: Default::default(),
            event_queue: tx,
            votes: Default::default(),
            state: State::NotStarted,
        }));

        tokio::spawn(Self::listen_events(rx, state.clone()));
        tokio::spawn(Self::fetch_updates(Arc::downgrade(&state)));

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
            votes: self.get_votes().ok(),
            state: self.state.clone(),
            roles: matches!(self.state, State::Finished)
                .then_some(
                    self.players
                        .values()
                        .map(|p| (p.session.name.clone(), p.role()))
                        .fold(Some(HashMap::new()), |map, (name, role)| {
                            match (map, role) {
                                (Some(mut map), Some(role)) => {
                                    map.insert(name, role);
                                    Some(map)
                                }
                                _ => None,
                            }
                        }),
                )
                .flatten(),
        }
    }

    /// Returns the public status of the game, along with the status of one player
    ///
    /// - puuid: Puuid of the player of which the status is requested
    pub async fn get_status_authenticated(
        &self,
        puuid: &String,
    ) -> Result<AuthenticatedGameStatus, Error> {
        let res = Ok(AuthenticatedGameStatus {
            uid: self.uid,
            player_names: self
                .players
                .values()
                .map(|p| p.session.name.clone())
                .collect(),
            has_started: self.data.read().await.is_some(),
            player_state: self.players.get(puuid).ok_or(Error::Unauthorized)?.state(),
            votes: self.get_votes().ok(),
            state: self.state.clone(),
            roles: matches!(self.state, State::Finished)
                .then_some(
                    self.players
                        .values()
                        .map(|p| (p.session.name.clone(), p.role()))
                        .fold(Some(HashMap::new()), |map, (name, role)| {
                            match (map, role) {
                                (Some(mut map), Some(role)) => {
                                    map.insert(name, role);
                                    Some(map)
                                }
                                _ => {
                                    None
                                },
                            }
                        }),
                )
                .flatten(),
        });
        res
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

    /// Start the game by creating and assigning roles
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

    pub fn add_votes(&mut self, name: String, votes: HashMap<String, Role>) -> Result<(), Error> {
        if self.votes.len() != 5 && !self.votes.contains_key(&name) {
            if let State::WaitingVotes { ref players } = self.state {
                self.votes.insert(name.clone(), votes);
                if self.votes.len() == 5 {
                    self.state = State::Finished
                } else {
                    self.state = State::WaitingVotes {
                        players: players.iter().filter(|p| p != &&name).cloned().collect(),
                    };
                }

                for p in self.players.values() {
                    p.proxy.send_message(Message::State {
                        state: self.state.clone(),
                    })
                }

                Ok(())
            } else {
                Err(Error::VotesClosed)
            }
        } else {
            Err(Error::VotesClosed)
        }
    }

    pub fn get_votes(&self) -> Result<HashMap<String, HashMap<String, Role>>, Error> {
        if self.votes.len() == 5 {
            Ok(self.votes.clone())
        } else {
            Err(Error::VotesNotReady)
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
            tokio::time::sleep(Duration::from_secs(10)).await;

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
                    match lol_api::spectator::get_active_game(summoner_id.clone()).await {
                        Ok(match_info) => {
                            let mut lock = state.write().await;
                            if matches!(lock.state, State::NotStarted | State::InGame) {
                                lock.update_state(match_info).await;
                                lock.state = State::InGame
                            }
                        }
                        Err(_) => {
                            let mut lock = state.write().await;
                            if lock.player_count() == 5
                                && matches!(lock.state, State::NotStarted | State::InGame)
                            {
                                lock.state = State::WaitingVotes {
                                    players: lock
                                        .players
                                        .values()
                                        .map(|p| p.session.name.clone())
                                        .collect(),
                                };
                                for p in lock.players.values() {
                                    p.proxy.send_message(Message::State {
                                        state: lock.state.clone(),
                                    })
                                }
                            }
                        }
                    }
                }
            } else {
                break;
            }
        }
    }
}
