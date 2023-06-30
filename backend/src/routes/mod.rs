use self::error::Error;
use crate::game::messages::Message;
use crate::session_management::UserSession;
use crate::AppState;
use rocket::get;
use rocket::serde::json::Json;

pub mod error;
pub mod game;

#[get("/login")]
pub async fn login(state: &AppState) -> Result<String, Error> {
    let state = state.lock().unwrap();
    let session = UserSession::default();
    state.get_or_create_proxy(session.uid);
    session.encode().map_err(Error::from)
}

#[get("/updates")]
pub async fn get_updates(
    session: UserSession,
    state: &AppState,
) -> Result<Json<Vec<Message>>, Error> {
    let state = state.lock().unwrap();

    let lock = state.messages.lock().unwrap();
    let Some(messages_mutex) = lock.get(&session.uid) else {
        return Err(Error::NotFound)
    };

    let mut vec = messages_mutex.messages.lock().unwrap();
    let messages = vec.clone();
    *vec = vec![];

    Ok(Json(messages))
}
