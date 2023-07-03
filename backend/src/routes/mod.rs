use self::error::Error;
use crate::game::messages::Message;
use crate::lol_api::summoners::Puuid;
use crate::session_management::UserSession;
use crate::AppState;
use rocket::{get, post};
use rocket::serde::json::Json;
use serde::Deserialize;

pub mod error;
pub mod game;

#[derive(Debug, Deserialize)]
pub struct LoginForm {
    puuid: Puuid,
}

#[post("/login", format="json", data="<login_form>")]
pub async fn login(state: &AppState, login_form: Json<LoginForm>) -> Result<String, Error> {
    let state = state.lock().await;
    let session = UserSession::new(login_form.puuid.clone());
    state.get_or_create_proxy(&session.puuid);
    session.encode().map_err(Error::from)
}

#[get("/updates")]
pub async fn get_updates(
    session: UserSession,
    state: &AppState,
) -> Result<Json<Vec<Message>>, Error> {
    let state = state.lock().await;

    let lock = state.messages.lock().unwrap();
    let Some(messages_mutex) = lock.get(&session.puuid) else {
        return Err(Error::NotFound)
    };

    let mut vec = messages_mutex.messages.lock().unwrap();
    let messages = vec.clone();
    *vec = vec![];

    Ok(Json(messages))
}
