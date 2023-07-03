#![allow(dead_code)] // TODO

use rocket::launch;
use tokio::sync::Mutex;

pub mod game;
pub mod lol_api;
pub mod models;
pub mod routes;
pub mod session_management;
pub mod state;

pub type AppState = rocket::State<Mutex<state::State>>;

#[launch]
async fn rocket() -> _ {

    rocket::build()
        .manage(Mutex::new(state::State::default()))
        .mount(
            "/",
            rocket::routes![
                routes::get_updates,
                routes::login,
                routes::game::get_game,
                routes::game::create_game,
                routes::game::join_game
            ],
        )
}
