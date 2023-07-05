use serde::{de::DeserializeOwned, Deserialize};

use crate::{env::env_config, routes::error::Error};

pub mod summoners;

#[derive(Deserialize, Debug)]
pub struct LolApiError {
    pub message: String,
    pub status_code: i32,
}

pub async fn make_api_call<T>(uri: String, parameters: &[(&str, &str)]) -> Result<T, Error>
where
    T: DeserializeOwned,
{
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum ApiResult<T> {
        Value(T),
        Error { status: LolApiError },
    }

    let result: ApiResult<T> = reqwest::Client::new()
        .get(format!(
            "https://{}.api.riotgames.com{}",
            env_config().lol_api_region.as_str(),
            uri
        ))
        .header("X-Riot-Token", env_config().lol_api_key.as_str())
        .query(parameters)
        .send()
        .await?
        .json()
        .await?;

    match result {
        ApiResult::Value(v) => Ok(v),
        ApiResult::Error { status } => Err(Error::from(status)),
    }
}
