use crate::game::messages::Message;
use crate::session_management::UserSession;
use crate::AppState;
use rocket::get;
use rocket::http::Status;
use rocket::serde::json::Json;
use std::sync::Arc;

pub mod error;

#[get("/login")]
pub async fn login(state: &AppState) -> Result<String, Status> {
    let session = UserSession::default();
    state
        .messages
        .lock()
        .unwrap()
        .insert(session.uid, Arc::default());
    session.encode().map_err(|_| Status::InternalServerError)
}

#[get("/updates")]
pub async fn get_updates(
    session: UserSession,
    state: &AppState,
) -> Result<Json<Vec<Message>>, Status> {
    let messages_lock = state.messages.lock().unwrap();

    let Some(messages_mutex) = messages_lock.get(&session.uid) else {
        return Err(Status::NotFound)
    };

    let mut vec = messages_mutex.lock().unwrap();
    let messages = vec.clone();
    *vec = vec![];

    Ok(Json(messages))
}
