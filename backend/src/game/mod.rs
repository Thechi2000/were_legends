use self::player::{proxy::PlayerProxy, Player};
use crate::{
    models::{MatchDto, MatchDtoMutation},
    routes::error::Error,
};
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

pub enum GameEvent {
    MatchDtoMutation(MatchDtoMutation),
    PlayerJoin(Uuid),
    GameStart,
}

pub struct GameState {
    players: HashMap<Uuid, Player>,
    data: RwLock<Option<MatchDto>>,
    event_queue: Sender<GameEvent>,
}

impl GameState {
    pub fn new() -> Arc<RwLock<Self>> {
        let (tx, rx) = tokio::sync::mpsc::channel(100);
        let state = Arc::new(RwLock::new(Self {
            players: Default::default(),
            data: Default::default(),
            event_queue: tx,
        }));

        tokio::spawn(Self::listen_events(rx, state.clone()));

        state
    }

    pub fn has_player(&self, uuid: Uuid) -> bool {
        self.players.contains_key(&uuid)
    }

    pub async fn add_player(&mut self, uid: Uuid, proxy: PlayerProxy) -> Result<(), Error> {
        if self.players.len() > 5 {
            Err(Error::MaxPlayerReached)
        } else if let hash_map::Entry::Vacant(e) = self.players.entry(uid) {
            e.insert(Player::new(proxy));
            self.event_queue.send(GameEvent::PlayerJoin(uid)).await?;
            Ok(())
        } else {
            Err(Error::AlreadyInGame)
        }
    }

    pub async fn listen_events(mut rx: Receiver<GameEvent>, state: Arc<RwLock<Self>>) {
        loop {
            while let Some(event) = rx.recv().await {
                match event {
                    GameEvent::PlayerJoin(uid) => {
                        for player in state.read().await.players.values() {
                            player
                                .proxy
                                .send_message(messages::Message::PlayerJoin(uid))
                        }
                    }
                    _ => todo!(),
                }
            }
        }
    }
}
