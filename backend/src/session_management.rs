use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation};
use rocket::{
    http::{Cookie, Status},
    request::{FromRequest, Outcome},
    Request,
};
use serde::{Deserialize, Serialize};

use crate::{env::env_config, routes::error::Error};

const SESSION_COOKIE_NAME: &str = "session";

/// Guard for the user's session
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserSession {
    pub name: String,
}

#[derive(Debug)]
pub enum UserSessionError {
    Missing,
    BadFormat,
    Invalid(jsonwebtoken::errors::Error),
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for UserSession {
    type Error = UserSessionError;

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        if let Some(header) = req.headers().get_one("Authorization") {
            let Some(header) = header.strip_prefix("Bearer ") else {
                return Outcome::Failure((Status::BadRequest, UserSessionError::BadFormat))
            };

            let mut validation = Validation::default();
            validation.validate_exp = false;
            validation.required_spec_claims.clear();

            match jsonwebtoken::decode::<UserSession>(
                header,
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
    /// Creates an encoded string for this session, using the secret stored in JWT_SECRET
    pub fn encode(&self) -> Result<String, jsonwebtoken::errors::Error> {
        jsonwebtoken::encode(
            &Header::default(),
            self,
            &EncodingKey::from_secret(env_config().jwt_secret.as_ref()),
        )
    }

    pub async fn new(name: String) -> Result<Self, Error> {
        if (1..16).contains(&name.len()) {
            Ok(Self { name })
        } else {
            Err(Error::InvalidName)
        }
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
