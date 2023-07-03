use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation};
use rocket::{
    http::Status,
    request::{FromRequest, Outcome},
    Request,
};
use serde::{Deserialize, Serialize};

use crate::lol_api::summoners::Puuid;

const SESSION_COOKIE_NAME: &str = "session";
const JWT_SECRET: &str = "best secret ever"; // TODO

#[derive(Serialize, Deserialize)]
pub struct UserSession {
    pub puuid: Puuid,
}

#[derive(Debug)]
pub enum UserSessionError {
    Missing,
    Invalid(jsonwebtoken::errors::Error),
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for UserSession {
    type Error = UserSessionError;

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        if let Some(cookie) = req.cookies().get(SESSION_COOKIE_NAME) {
            let mut validation = Validation::default();
            validation.validate_exp = false;
            validation.required_spec_claims.clear();

            match jsonwebtoken::decode::<UserSession>(
                cookie.value(),
                &DecodingKey::from_secret(JWT_SECRET.as_ref()),
                &validation,
            ) {
                Ok(session) => Outcome::Success(session.claims),
                Err(e) => Outcome::Failure((Status::BadRequest, UserSessionError::Invalid(e))),
            }
        } else {
            Outcome::Failure((Status::Forbidden, UserSessionError::Missing))
        }
    }
}

impl UserSession {
    pub fn encode(&self) -> Result<String, jsonwebtoken::errors::Error> {
        jsonwebtoken::encode(
            &Header::default(),
            self,
            &EncodingKey::from_secret(JWT_SECRET.as_ref()),
        )
    }

    pub fn new(puuid: Puuid) -> Self {
        Self {
            puuid,
        }
    }
}
