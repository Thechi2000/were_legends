use serde::Serialize;

use super::player::classes::bot::Mission;

/// Messages that can be sent to players through the [PlayerProxy] interface
#[derive(Serialize, Debug, Clone)]
pub enum Message {
    Hi,
    PlayerJoin {
        name: String,
    },
    Debug {
        // TODO
        value: String,
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
