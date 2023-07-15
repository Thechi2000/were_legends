use rocket::{get, post, serde::json::Json};
use uuid::Uuid;

use crate::{
    game::{AuthenticatedGameStatus, GameStatus},
    models::AllGameData,
    session_management::UserSession,
    AppState,
};

use super::error::Error;

#[get("/game", rank = 0)]
pub async fn get_current_game_authenticated(
    player: UserSession,
    state: &AppState,
) -> Result<Json<AuthenticatedGameStatus>, Error> {
    Ok(Json(
        state
            .lock()
            .await
            .get_game_by_player(&player.puuid)
            .await
            .ok_or(Error::NotFound)?
            .1
            .read()
            .await
            .get_status_authenticated(&player.puuid)
            .await?,
    ))
}

#[get("/game/<uid>", rank = 1)]
pub async fn get_game_authenticated(
    player: UserSession,
    state: &AppState,
    uid: Uuid,
) -> Result<Json<AuthenticatedGameStatus>, Error> {
    Ok(Json(
        state
            .lock()
            .await
            .games
            .get(&uid)
            .ok_or(Error::NotFound)?
            .read()
            .await
            .get_status_authenticated(&player.puuid)
            .await?,
    ))
}

#[get("/game/<uid>", rank = 2)]
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

#[post("/game/update", format = "json", data = "<data>")]
pub async fn update_game(
    player: UserSession,
    state: &AppState,
    data: Json<AllGameData>,
) -> Result<(), Error> {
    let lock = state.lock().await;
    let game = lock
        .get_game_by_player(&player.puuid)
        .await
        .ok_or(Error::NotInGame)?;
    drop(lock);

    game.1.write().await.update_state(data.into_inner()).await;

    Ok(())
}

#[post("/game/quit")]
pub async fn quit_game(player: UserSession, state: &AppState) -> Result<(), Error> {
    let game = state.lock().await
        .get_game_by_player(&player.puuid)
        .await
        .ok_or(Error::NotInGame)?;

    game.1.write().await.remove_player(player.puuid).await?;
    state.lock().await.try_remove_game(game.0).await;

    Ok(())
}
