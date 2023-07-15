use serde::Serialize;

use super::{player::classes::droid::Mission, team_builder::Role};

/// Messages that can be sent to players through the [PlayerProxy] interface
#[derive(Serialize, Debug, Clone)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Message {
    Hi,
    PlayerJoin {
        name: String,
    },
    Debug {
        // TODO
        value: String,
    },
    Role {
        role: Role,
    },
    Mission {
        mission: Mission,
    },
    Juliette {
        name: String,
    },
    TwoFaceState {
        inting: bool,
    },
}
