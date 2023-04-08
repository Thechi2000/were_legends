use rocket::launch;

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
