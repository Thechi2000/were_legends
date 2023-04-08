use serde::Serialize;

#[derive(Serialize, Debug)]
pub enum Error {
    NoSuchPlayer,
}
