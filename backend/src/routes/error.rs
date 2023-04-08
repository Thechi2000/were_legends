use rocket::response::Responder;
use serde::Serialize;

#[derive(Serialize, Debug)]
pub enum Error {
    NoSuchPlayer,
}
