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

    dbg!(lol_api::summoners::get_by_puuid("QLJqmSUMrZCH3vzhr_MxWoRh2sBDXq_sMhGl7bUEP0eM9WhDMCoK3ur43l-JStqn_7Quo2akh6CVMg".into()).await).unwrap();

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
