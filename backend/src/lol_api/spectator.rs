use mutable::cmp::SoftEq;
use mutable::{Mutable, SoftEq};
use serde::Deserialize;

use crate::routes::error::Error;

use super::make_api_call;

#[derive(Debug, Deserialize, Default, PartialEq, Mutable)]
#[serde(rename_all = "camelCase")]
pub struct CurrentGameInfo {
    /// The ID of the game
    pub game_id: i64,
    /// The game type
    pub game_type: String,
    /// The game start time represented in epoch milliseconds
    pub game_start_time: i64,
    /// The ID of the map
    pub map_id: i64,
    /// The amount of time in seconds that has passed since the game started
    pub game_length: i64,
    /// The ID of the platform on which the game is being played
    pub platform_id: String,
    /// The game mode
    pub game_mode: String,
    /// Banned champion information
    pub banned_champions: Vec<BannedChampion>,
    /// The queue type (queue types are documented on the Game Constants page)
    pub game_queue_config_id: i64,
    /// The observer information
    pub observers: Observer,
    /// The participant information
    pub participants: Vec<CurrentGameParticipant>,
}

#[derive(Debug, Deserialize, Default, PartialEq, Mutable, SoftEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct BannedChampion {
    /// The turn during which the champion was banned
    pub pick_turn: i32,
    /// The ID of the banned champion
    #[softeq(uid)]
    pub champion_id: i64,
    /// The ID of the team that banned the champion
    pub team_id: i64,
}

#[derive(Debug, Deserialize, Default, PartialEq, Mutable)]
#[serde(rename_all = "camelCase")]
pub struct Observer {
    /// Key used to decrypt the spectator grid game data for playback
    pub encryption_key: String,
}

#[derive(Debug, Deserialize, Default, PartialEq, Mutable, SoftEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CurrentGameParticipant {
    /// The ID of the champion played by this participant
    pub champion_id: i64,
    /// Perks/Runes Reforged Information
    pub perks: Perks,
    /// The ID of the profile icon used by this participant
    pub profile_icon_id: i64,
    /// Flag indicating whether or not this participant is a bot
    pub bot: bool,
    /// The team ID of this participant, indicating the participant's team
    pub team_id: i64,
    /// The summoner name of this participant
    pub summoner_name: String,
    /// The encrypted summoner ID of this participant
    #[softeq(uid)]
    pub summoner_id: String,
    /// The ID of the first summoner spell used by this participant
    pub spell1_id: i64,
    /// The ID of the second summoner spell used by this participant
    pub spell2_id: i64,
    /// List of Game Customizations
    pub game_customization_objects: Vec<GameCustomizationObject>,
}

#[derive(Debug, Deserialize, Default, PartialEq, Mutable, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Perks {
    /// IDs of the perks/runes assigned.
    pub perk_ids: Vec<i64>,
    /// Primary runes path
    pub perk_style: i64,
    /// Secondary runes path
    pub perk_sub_style: i64,
}

#[derive(Debug, Deserialize, Default, PartialEq, Mutable, SoftEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GameCustomizationObject {
    /// Category identifier for Game Customization
    #[softeq(uid)]
    pub category: String,
    /// Game Customization content
    pub content: String,
}

/* lazy_static! {
    static ref DIR_READER: tokio::sync::Mutex<<Vec<std::fs::DirEntry> as IntoIterator>::IntoIter> = {
        let mut vec: Vec<_> = std::fs::read_dir("data")
            .unwrap()
            .filter_map(Result::ok)
            .collect();
        vec.sort_by_key(|e| e.file_name());
        tokio::sync::Mutex::new(vec.into_iter())
    };
} */

pub async fn get_active_game(summoner_id: String) -> Result<CurrentGameInfo, Error> {
    make_api_call(
        format!("/lol/spectator/v4/active-games/by-summoner/{summoner_id}"),
        &[],
        false,
    )
    .await
    /*  if let Some(entry) = DIR_READER.lock().await.nth(10) {
        let res = serde_json::from_str::<CurrentGameInfo>(
            tokio::fs::read_to_string(entry.path())
                .await
                .unwrap()
                .as_str(),
        )
        .unwrap();
        Ok(res)
    } else {
        Err(Error::Internal {
            msg: "not found".into(),
        })
    } */
}
