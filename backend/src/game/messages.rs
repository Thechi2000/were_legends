use serde::Serialize;

#[derive(Serialize, Debug, Clone)]
pub enum Message {
    Hi,
    PlayerJoin{
        name: String,
    },
    Debug { // TODO
        value: String
    }
}
