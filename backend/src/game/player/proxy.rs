use crate::game::messages::Message;
use std::sync::{Arc, Mutex};

pub struct PlayerProxy {
    messages: Arc<Mutex<Vec<Message>>>,
}

impl PlayerProxy {
    pub fn send_message(&self, msg: Message) {
        self.messages.as_ref().lock().unwrap().push(msg)
    }
}
