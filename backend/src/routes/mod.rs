use crate::game::messages::{Message, self};
use crate::AppState;
use rocket::get;
use rocket::http::Status;
use rocket::serde::json::Json;
use uuid::Uuid;

pub mod error;

#[get("/updates/<uid>")]
pub async fn get_updates(uid: Uuid, state: &AppState) -> Result<Json<Vec<Message>>, Status> {
    let Some(messages_mutex) = state.messages.get(&uid) else {
        return Err(Status::NotFound)
    };

    let mut vec = messages_mutex.lock().unwrap();
    let messages = vec.clone();
    *vec = vec![];

    Ok(Json(messages))
}
