use rocket::{get, post, serde::json::Json};
use uuid::Uuid;

use crate::{game::GameStatus, session_management::UserSession, AppState};

use super::error::Error;

#[get("/game/<uid>")]
pub async fn get_game(state: &AppState, uid: Uuid) -> Result<Json<GameStatus>, Error> {
    Ok(Json(
        state
            .lock()
            .await
            .games
            .get(&uid)
            .ok_or(Error::NotFound)?
            .read()
            .await
            .get_status()
            .await,
    ))
}

#[post("/game")]
pub async fn create_game(player: UserSession, state: &AppState) -> Result<Json<Uuid>, Error> {
    let mut lock = state.lock().await;

    if lock.get_game_by_player(&player.puuid).await.is_some() {
        return Err(Error::AlreadyInGame);
    }

    let proxy = lock.get_or_create_proxy(&player.puuid);
    let (uid, game) = lock.create_game();

    drop(lock);

    game.write().await.add_player(player.puuid, proxy).await?;

    Ok(Json(uid))
}

#[post("/game/<uid>/join")]
pub async fn join_game(player: UserSession, state: &AppState, uid: Uuid) -> Result<(), Error> {
    let lock = state.lock().await;
    let proxy = lock.get_or_create_proxy(&player.puuid);
    let game = lock.get_game_by_id(uid).ok_or(Error::NotFound)?;
    drop(lock);

    game.write().await.add_player(player.puuid, proxy).await?;

    Ok(())
}
