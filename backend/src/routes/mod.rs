use self::error::Error;
use crate::game::messages::Message;
use crate::session_management::UserSession;
use crate::AppState;
use rocket::serde::json::Json;
use rocket::{get, post};
use serde::Deserialize;

pub mod error;
pub mod game;

#[derive(Debug, Deserialize)]
pub struct LoginForm {
    name: String,
}

#[post("/login", format = "json", data = "<login_form>")]
pub async fn login(state: &AppState, login_form: Json<LoginForm>) -> Result<String, Error> {
    let state = state.lock().await;
    let session = UserSession::new(login_form.name.clone());
    state.get_or_create_proxy(&session.name);

    session.encode().map_err(Error::from)
}

#[get("/updates")]
pub async fn get_updates(
    session: UserSession,
    state: &AppState,
) -> Result<Json<Vec<Message>>, Error> {
    let state = state.lock().await;

    let lock = state.messages.lock().unwrap();
    let Some(messages_mutex) = lock.get(&session.name) else {
        return Err(Error::NotFound)
    };

    let mut vec = messages_mutex.messages.lock().unwrap();
    let messages = vec.clone();
    *vec = vec![];

    Ok(Json(messages))
}
