use serde::Deserialize;

use crate::routes::error::Error;

use super::make_api_call;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SummonerByPuuid {
    pub puuid: String,
    pub game_name: String,
    pub tag_line: String,
}
pub async fn get_by_puuid(puuid: String) -> Result<SummonerByPuuid, Error> {
    make_api_call(format!("/riot/account/v1/accounts/by-puuid/{puuid}"), &[], true).await
}
