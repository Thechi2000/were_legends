use serde::Serialize;

use super::{player::classes::{droid::Mission, romeo::Juliette}, team_builder::Role, PublicInnerState};

/// Messages that can be sent to players through the [PlayerProxy] interface
#[derive(Serialize, Debug, Clone)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Message {
    Hi,
    PlayerJoin {
        name: String,
    },
    Role {
        role: Role,
    },
    Mission {
        mission: Mission,
    },
    Juliette {
        #[serde(flatten)]
        juliette: Juliette,
    },
    TwoFaceState {
        inting: bool,
    },
    VotesCompleted,
    State {
        state: PublicInnerState,
    },
}
