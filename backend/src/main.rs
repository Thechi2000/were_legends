#![allow(dead_code)] // TODO

use env::env_config;
use rocket::launch;
use tokio::sync::Mutex;

pub mod env;
pub mod game;
// pub mod lol_api;
pub mod models;
pub mod routes;
pub mod session_management;
pub mod state;

pub type AppState = rocket::State<Mutex<state::State>>;

#[launch]
async fn rocket() -> _ {
    env::env_config();

    rocket::build()
        .manage(Mutex::new(state::State::default()))
        .mount(
            &env_config().uri,
            rocket::routes![
                routes::get_updates,
                routes::login,
                routes::game::get_game,
                routes::game::get_game_authenticated,
                routes::game::get_current_game_authenticated,
                routes::game::create_game,
                routes::game::join_game,
                routes::game::quit_game,
                routes::game::start_game,
                routes::game::post_votes,
            ],
        )
}
