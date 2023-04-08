use std::{str::FromStr, sync::Mutex};

use game::messages::Message;
use rocket::launch;
use uuid::Uuid;

pub mod game;
pub mod models;
pub mod routes;
pub mod state;

pub type AppState = rocket::State<state::State>;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .manage(state::State::default())
        .mount("/", rocket::routes![routes::get_updates])
}
