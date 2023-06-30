#![allow(dead_code)] // TODO

use std::sync::Mutex;

use rocket::launch;

pub mod game;
pub mod models;
pub mod routes;
pub mod session_management;
pub mod state;

pub type AppState = rocket::State<Mutex<state::State>>;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .manage(Mutex::new(state::State::default()))
        .mount(
            "/",
            rocket::routes![
                routes::get_updates,
                routes::login,
                routes::game::create_game,
                routes::game::join_game
            ],
        )
}
