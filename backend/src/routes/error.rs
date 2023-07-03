use rocket::{
    http::{ContentType, Status},
    response::Responder,
    Response,
};
use serde::Serialize;
use std::{fmt::Debug, io::Cursor};

#[derive(Serialize, Debug)]
#[serde(tag = "error")]
pub enum Error {
    NotFound,
    NotInGame,
    AlreadyInGame,
    MaxPlayerReached,
    Internal { msg: String },
}

impl<'r, 'o: 'r> Responder<'r, 'o> for Error {
    fn respond_to(self, _: &'r rocket::Request<'_>) -> rocket::response::Result<'o> {
        let status = match self {
            Error::NotFound => Status::NotFound,
            Error::Internal { .. } => Status::InternalServerError,
            Error::AlreadyInGame => Status::BadRequest,
            Error::NotInGame => Status::BadRequest,
            Error::MaxPlayerReached => Status::BadRequest,
        };
        let Ok(body) = serde_json::to_string(&self) else {
            return Err(Status::InternalServerError)
        };

        Response::build()
            .status(status)
            .header(ContentType::JSON)
            .sized_body(body.len(), Cursor::new(body))
            .ok()
    }
}

impl Error {
    pub fn from<T: Debug>(e: T) -> Self {
        Self::Internal {
            msg: format!("{:?}", e),
        }
    }
}

impl<T> From<tokio::sync::mpsc::error::SendError<T>> for Error {
    fn from(e: tokio::sync::mpsc::error::SendError<T>) -> Self {
        Self::Internal {
            msg: format!("{:?}", e),
        }
    }
}

impl From<reqwest::Error> for Error {
    fn from(e: reqwest::Error) -> Self {
        Self::Internal {
            msg: format!("{:?}", e),
        }
    }
}
