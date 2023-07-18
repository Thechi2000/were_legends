use self::{
    player::{classes::PlayerState, proxy::PlayerProxy, Player},
    team_builder::Role,
};
use crate::{routes::error::Error, session_management::UserSession};
use mutable::Mutable;
use serde::Serialize;
use std::{
    collections::{hash_map, HashMap},
    sync::{Arc, Weak},
    time::{Duration, Instant},
};
use tokio::sync::RwLock;
use uuid::Uuid;

pub mod messages;
pub mod player;
pub mod team_builder;

macro_rules! require_state {
    ($state:pat, $self:expr) => {
        let $state = $self.state else { return Err(Error::IncorrectState); };
    };
}

/// Public status of a game
#[derive(Debug, Serialize)]
pub struct GameStatus {
    uid: Uuid,
    player_names: Vec<String>,
    #[serde(flatten)]
    state: PublicInnerState,
}

/// Public status of a game, augmented with the state of the player
#[derive(Debug, Serialize)]
pub struct AuthenticatedGameStatus {
    uid: Uuid,
    player_names: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    player_state: Option<PlayerState>,
    #[serde(flatten)]
    state: PublicInnerState,
}

/// State of a game
pub struct GameState {
    uid: Uuid,
    players: HashMap<String, Player>,
    state: InnerState,
}

#[derive(Debug, Mutable, Default)]
pub struct GameInfo {
    pub duration: u64,
}

#[derive(Debug)]
enum InnerState {
    Setup,
    Draft,
    InGame {
        start: Instant,
        data: Option<GameInfo>,
    },
    Voting {
        votes: HashMap<String, HashMap<String, Role>>,
    },
    End {
        votes: HashMap<String, HashMap<String, Role>>,
    },
}

#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "snake_case", tag = "state")]
pub enum PublicInnerState {
    Setup,
    Draft,
    InGame,
    Voting {
        votes_received: Vec<String>,
    },
    End {
        votes: HashMap<String, HashMap<String, Role>>,
        roles: HashMap<String, Role>,
    },
}

impl PublicInnerState {
    fn try_from(value: &InnerState, players: &HashMap<String, Player>) -> Result<Self, Error> {
        fn convert_players(map: &HashMap<String, Player>) -> Vec<String> {
            map.values().map(|p| p.session.name.clone()).collect()
        }
        Ok(match value {
            InnerState::Setup => Self::Setup,
            InnerState::Draft => Self::Draft,
            InnerState::InGame { .. } => Self::InGame,
            InnerState::Voting { votes } => Self::Voting {
                votes_received: votes.keys().cloned().collect(),
            },
            InnerState::End { votes } => Self::End {
                votes: votes.clone(),
                roles: players
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
                    })
                    .ok_or(Error::Internal {
                        msg: "missing role".into(),
                    })?,
            },
        })
    }
}

impl GameState {
    pub fn new(uid: Uuid) -> Arc<RwLock<Self>> {
        let state = Arc::new(RwLock::new(Self {
            uid,
            players: Default::default(),
            state: InnerState::Setup {},
        }));

        // tokio::spawn(Self::listen_events(rx, state.clone()));
        tokio::spawn(Self::fetch_updates(Arc::downgrade(&state)));

        state
    }

    /// Returns the public status of the game
    pub async fn get_status(&self) -> Result<GameStatus, Error> {
        Ok(GameStatus {
            uid: self.uid,
            player_names: self
                .players
                .values()
                .map(|p| p.session.name.clone())
                .collect(),
            state: PublicInnerState::try_from(&self.state, &self.players)?,
        })
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
            player_state: self.players.get(puuid).ok_or(Error::Unauthorized)?.state(),
            state: PublicInnerState::try_from(&self.state, &self.players)?,
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
        require_state!(InnerState::Setup, self);

        if self.player_count() == 5 {
            return Err(Error::MaxPlayerReached);
        }

        if let hash_map::Entry::Vacant(e) = self.players.entry(session.name.clone()) {
            e.insert(Player::new(session.clone(), proxy));
        }

        Ok(())
    }

    /// Remove a player from this game
    ///
    /// - puuid: Puuid of the player to remove
    pub async fn remove_player(&mut self, puuid: String) -> Result<(), Error> {
        match self.state {
            InnerState::Setup | InnerState::End { .. } => {
                if let Some(_) = self.players.remove(&puuid) {
                    Ok(())
                } else {
                    Err(Error::NotInGame)
                }
            }
            _ => Err(Error::IncorrectState),
        }
    }

    /// Update the state of the game with data from the LoL Client API
    pub async fn update_state(&mut self, game_info: GameInfo) {
        if let InnerState::InGame { ref mut data, .. } = self.state {
            if data.is_none() {
                *data = Some(GameInfo::default());
            }

            let mutations = data.as_mut().unwrap().update(game_info);
            for mutation in mutations {
                for player in self.players.values_mut() {
                    if let Err(e) = player.receive_mutation(&mutation, data.as_ref().unwrap()) {
                        tracing::error!("Could not process mutation: {:?}", e)
                    }
                }
            }
        }
    }

    /// Start the game by creating and assigning roles
    pub async fn start(&mut self) -> Result<(), Error> {
        match self.state {
            InnerState::Setup => {
                if self.player_count() != 5 {
                    return Err(Error::NotEnoughPlayers);
                }

                let composition = team_builder::generate_composition();

                for (player, role) in self.players.values_mut().zip(composition.iter()) {
                    player.set_role(*role)?;
                }

                self.state = InnerState::Draft {};
            }
            InnerState::Draft => {
                self.state = InnerState::InGame {
                    start: Instant::now(),
                    data: None,
                }
            }
            _ => return Err(Error::IncorrectState),
        }
        Ok(())
    }

    pub async fn end(&mut self) -> Result<(), Error> {
        if let InnerState::InGame { start, .. } = self.state {
            if start.elapsed().as_secs() > 10 {
                self.state = InnerState::Voting {
                    votes: Default::default(),
                };
                Ok(())
            } else {
                Err(Error::IncorrectState)
            }
        } else {
            Err(Error::IncorrectState)
        }
    }

    pub async fn add_votes(
        &mut self,
        name: String,
        ballots: HashMap<String, Role>,
    ) -> Result<(), Error> {
        match self.state {
            InnerState::Voting { ref mut votes } => {
                if votes.len() != 5 && !votes.contains_key(&name) {
                    votes.insert(name.clone(), ballots);

                    if votes.len() == 5 {
                        self.state = InnerState::End {
                            votes: votes.clone(),
                        }
                    }

                    Ok(())
                } else {
                    Err(Error::VotesClosed)
                }
            }
            _ => Err(Error::IncorrectState),
        }
    }

    /*
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
    } */

    async fn fetch_updates(state: Weak<RwLock<Self>>) {
        loop {
            tokio::time::sleep(Duration::from_secs(3)).await;

            if let Some(state) = state.upgrade() {
                let mut lock = state.write().await;
                if let InnerState::InGame { start, .. } = lock.state {
                    lock.update_state(GameInfo {
                        duration: start.elapsed().as_secs(),
                    })
                    .await;
                }
            }
        }
    }
}
