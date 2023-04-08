use serde::Serialize;

#[derive(Serialize, Debug, Clone)]
pub enum Message {
    Hi,
}
