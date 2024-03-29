use std::collections::HashMap;

use rocket::{get, post, serde::json::Json};
use uuid::Uuid;

use crate::{
    game::{team_builder::Role, AuthenticatedGameStatus, GameStatus},
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
            .get_game_by_player(&player.name)
            .await
            .ok_or(Error::NotFound)?
            .1
            .read()
            .await
            .get_status_authenticated(&player.name)
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
            .get_status_authenticated(&player.name)
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
            .await?,
    ))
}

#[post("/game")]
pub async fn create_game(player: UserSession, state: &AppState) -> Result<Json<Uuid>, Error> {
    let mut lock = state.lock().await;

    if lock.get_game_by_player(&player.name).await.is_some() {
        return Err(Error::AlreadyInGame);
    }

    let proxy = lock.get_or_create_proxy(&player.name);
    let (uid, game) = lock.create_game();

    drop(lock);

    game.write().await.add_player(player, proxy).await?;

    Ok(Json(uid))
}

#[post("/game/<uid>/join")]
pub async fn join_game(player: UserSession, state: &AppState, uid: Uuid) -> Result<(), Error> {
    let lock = state.lock().await;
    let proxy = lock.get_or_create_proxy(&player.name);
    let game = lock.get_game_by_id(uid).ok_or(Error::NotFound)?;
    drop(lock);

    game.write().await.add_player(player, proxy).await?;

    Ok(())
}

#[post("/game/quit")]
pub async fn quit_game(player: UserSession, state: &AppState) -> Result<(), Error> {
    let game = state
        .lock()
        .await
        .get_game_by_player(&player.name)
        .await
        .ok_or(Error::NotInGame)?;

    game.1.write().await.remove_player(player.name).await?;
    state.lock().await.try_remove_game(game.0).await;

    Ok(())
}

#[post("/game/start")]
pub async fn start_game(player: UserSession, state: &AppState) -> Result<(), Error> {
    let game = state
        .lock()
        .await
        .get_game_by_player(&player.name)
        .await
        .ok_or(Error::NotInGame)?;

    game.1.write().await.start().await?;

    Ok(())
}

#[post("/game/end")]
pub async fn end_game(player: UserSession, state: &AppState) -> Result<(), Error> {
    let game = state
        .lock()
        .await
        .get_game_by_player(&player.name)
        .await
        .ok_or(Error::NotInGame)?;

    game.1.write().await.end().await?;

    Ok(())
}

#[post("/game/votes", format = "json", data = "<votes>")]
pub async fn post_votes(
    player: UserSession,
    state: &AppState,
    votes: Json<HashMap<String, Role>>,
) -> Result<(), Error> {
    let game = state
        .lock()
        .await
        .get_game_by_player(&player.name)
        .await
        .ok_or(Error::NotInGame)?;

    game.1
        .write()
        .await
        .add_votes(player.name, votes.into_inner())
        .await?;

    Ok(())
}
