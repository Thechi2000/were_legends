use crate::game::messages::Message;
use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct PlayerProxy {
    pub messages: Arc<Mutex<Vec<Message>>>,
}

impl PlayerProxy {
    pub fn send_message(&self, msg: Message) {
        self.messages.as_ref().lock().unwrap().push(msg)
    }
}

impl Default for PlayerProxy {
    fn default() -> Self {
        Self {
            messages: Arc::new(Mutex::new(vec![Message::Hi])),
        }
    }
}
