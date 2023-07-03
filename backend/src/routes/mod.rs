use self::error::Error;
use crate::game::messages::Message;
use crate::lol_api::summoners::Puuid;
use crate::session_management::UserSession;
use crate::AppState;
use rocket::http::CookieJar;
use rocket::serde::json::Json;
use rocket::{get, post};
use serde::Deserialize;

pub mod error;
pub mod game;

#[derive(Debug, Deserialize)]
pub struct LoginForm {
    puuid: Puuid,
}

#[post("/login", format = "json", data = "<login_form>")]
pub async fn login(
    state: &AppState,
    login_form: Json<LoginForm>,
    cookies: &CookieJar<'_>,
) -> Result<(), Error> {
    let state = state.lock().await;
    let session = UserSession::new(login_form.puuid.clone());
    state.get_or_create_proxy(&session.puuid);

    cookies.add(session.try_into().map_err(Error::from)?);

    Ok(())
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
