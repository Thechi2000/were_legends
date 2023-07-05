use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation};
use rocket::{
    http::{Cookie, Status},
    request::{FromRequest, Outcome},
    Request,
};
use serde::{Deserialize, Serialize};

use crate::{env::env_config, lol_api::summoners::Puuid};

const SESSION_COOKIE_NAME: &str = "session";

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
                &DecodingKey::from_secret(env_config().jwt_secret.as_ref()),
                &validation,
            ) {
                Ok(session) => Outcome::Success(session.claims),
                Err(e) => Outcome::Failure((Status::BadRequest, UserSessionError::Invalid(e))),
            }
        } else {
            Outcome::Forward(())
        }
    }
}

impl UserSession {
    pub fn encode(&self) -> Result<String, jsonwebtoken::errors::Error> {
        jsonwebtoken::encode(
            &Header::default(),
            self,
            &EncodingKey::from_secret(env_config().jwt_secret.as_ref()),
        )
    }

    pub fn new(puuid: Puuid) -> Self {
        Self { puuid }
    }
}

impl TryFrom<UserSession> for Cookie<'_> {
    type Error = UserSessionError;

    fn try_from(value: UserSession) -> Result<Self, Self::Error> {
        Ok(Cookie::new(
            SESSION_COOKIE_NAME,
            value.encode().map_err(UserSessionError::Invalid)?,
        ))
    }
}
