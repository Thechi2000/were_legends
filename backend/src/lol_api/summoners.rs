use serde::Deserialize;

use crate::routes::error::Error;

use super::make_api_call;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SummonerByName {
    /// Encrypted account ID. Max length 56 characters. 
    pub account_id: String,
    /// ID of the summoner icon associated with the summoner. 
    pub profile_icon_id: i32,
    /// Date summoner was last modified specified as epoch milliseconds. The following events will update this timestamp: summoner name change, summoner level change, or profile icon change. 
    pub revision_date: i64,
    /// Summoner name. 
    pub name: String,
    /// Encrypted summoner ID. Max length 63 characters. 
    pub id: String,
    /// Encrypted PUUID. Exact length of 78 characters. 
    pub puuid: String,
    /// Summoner level associated with the summoner. 
    pub summoner_level: i64,
}
pub async fn get_by_name(name: String) -> Result<SummonerByName, Error> {
    make_api_call(format!("/lol/summoner/v4/summoners/by-name/{name}"), &[],false).await
}
