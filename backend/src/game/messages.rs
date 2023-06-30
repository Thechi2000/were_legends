use serde::Serialize;
use uuid::Uuid;

#[derive(Serialize, Debug, Clone)]
pub enum Message {
    Hi,
    PlayerJoin(Uuid),
}
