use mutable::{Mutable, SoftEq};
use mutable::cmp::SoftEq;
use serde::Deserialize;

#[derive(Deserialize, Debug, Clone, PartialEq, Mutable)]
#[serde(rename_all = "camelCase")]
pub struct MatchDto {
    /// Match metadata.
    metadata: MetadataDto,
    /// Match info.
    info: InfoDto,
}

#[derive(Deserialize, Debug, Clone, PartialEq, Mutable)]
#[serde(rename_all = "camelCase")]
pub struct MetadataDto {
    /// Match data version.
    data_version: String,
    /// Match id.
    match_id: String,
    /// A list of participant PUUIDs.
    participants: Vec<String>,
}

#[derive(Deserialize, Debug, Clone, PartialEq, Mutable)]
#[serde(rename_all = "camelCase")]
pub struct InfoDto {
    /// Unix timestamp for when the game is created on the game server (i.e., the loading screen).
    game_creation: i64,
    /// Prior to patch 11.20, this field returns the game length in milliseconds calculated from gameEndTimestamp - gameStartTimestamp. Post patch 11.20, this field returns the max timePlayed of any participant in the game in seconds, which makes the behavior of this field consistent with that of match-v4. The best way to handling the change in this field is to treat the value as milliseconds if the gameEndTimestamp field isn't in the response and to treat the value as seconds if gameEndTimestamp is in the response.
    game_duration: i64,
    /// Unix timestamp for when match ends on the game server. This timestamp can occasionally be significantly i64er than when the match "ends". The most reliable way of determining the timestamp for the end of the match would be to add the max time played of any participant to the gameStartTimestamp. This field was added to match-v5 in patch 11.20 on Oct 5th, 2021.
    game_end_timestamp: i64,
    game_id: i64,
    /// Refer to the Game Constants documentation.
    game_mode: String,
    game_name: String,
    /// Unix timestamp for when match starts on the game server.
    game_start_timestamp: i64,
    game_type: String,
    /// The first two parts can be used to determine the patch a game was played on.
    game_version: String,
    /// Refer to the Game Constants documentation.
    map_id: i32,
    participants: Vec<ParticipantDto>,
    /// Platform where the match was played.
    platform_id: String,
    /// Refer to the Game Constants documentation.
    queue_id: i32,
    teams: Vec<TeamDto>,
    /// Tournament code used to generate the match. This field was added to match-v5 in patch 11.13 on June 23rd, 2021.
    tournament_code: String,
}

#[derive(Deserialize, Debug, Clone, PartialEq, Mutable, SoftEq)]
#[serde(rename_all = "camelCase")]
pub struct ParticipantDto {
    assists: i32,
    baron_kills: i32,
    bounty_level: i32,
    champ_experience: i32,
    champ_level: i32,
    /// Prior to patch 11.4, on Feb 18th, 2021, this field returned invalid championIds. We recommend determining the champion based on the championName field for matches played prior to patch 11.4.
    champion_id: i32,
    champion_name: String,
    /// This field is currently only utilized for Kayn's transformations. (Legal values: 0 - None, 1 - Slayer, 2 - Assassin)
    champion_transform: i32,
    consumables_purchased: i32,
    damage_dealt_to_buildings: i32,
    damage_dealt_to_objectives: i32,
    damage_dealt_to_turrets: i32,
    damage_self_mitigated: i32,
    deaths: i32,
    detector_wards_placed: i32,
    double_kills: i32,
    dragon_kills: i32,
    first_blood_assist: bool,
    first_blood_kill: bool,
    first_tower_assist: bool,
    first_tower_kill: bool,
    game_ended_in_early_surrender: bool,
    game_ended_in_surrender: bool,
    gold_earned: i32,
    gold_spent: i32,
    /// Both individualPosition and teamPosition are computed by the game server and are different versions of the most likely position played by a player. The individualPosition is the best guess for which position the player actually played in isolation of anything else. The teamPosition is the best guess for which position the player actually played if we add the constrai32 that each team must have one top player, one jungle, one middle, etc. Generally the recommendation is to use the teamPosition field over the individualPosition field.
    individual_position: String,
    inhibitor_kills: i32,
    inhibitor_takedowns: i32,
    inhibitors_lost: i32,
    item0: i32,
    item1: i32,
    item2: i32,
    item3: i32,
    item4: i32,
    item5: i32,
    item6: i32,
    items_purchased: i32,
    killing_sprees: i32,
    kills: i32,
    lane: String,
    largest_critical_strike: i32,
    largest_killing_spree: i32,
    largest_multi_kill: i32,
    i64est_time_spent_living: i32,
    magic_damage_dealt: i32,
    magic_damage_dealt_to_champions: i32,
    magic_damage_taken: i32,
    neutral_minions_killed: i32,
    nexus_kills: i32,
    nexus_takedowns: i32,
    nexus_lost: i32,
    objectives_stolen: i32,
    objectives_stolen_assists: i32,
    participant_id: i32,
    penta_kills: i32,
    perks: PerksDto,
    physical_damage_dealt: i32,
    physical_damage_dealt_to_champions: i32,
    physical_damage_taken: i32,
    profile_icon: i32,
    puuid: String,
    quadra_kills: i32,
    riot_id_name: String,
    riot_id_tagline: String,
    role: String,
    sight_wards_bought_in_game: i32,
    spell1_casts: i32,
    spell2_casts: i32,
    spell3_casts: i32,
    spell4_casts: i32,
    summoner1_casts: i32,
    summoner1_id: i32,
    summoner2_casts: i32,
    summoner2_id: i32,
    #[softeq(uid)]
    summoner_id: String,
    summoner_level: i32,
    summoner_name: String,
    team_early_surrendered: bool,
    team_id: i32,
    /// Both individualPosition and teamPosition are computed by the game server and are different versions of the most likely position played by a player. The individualPosition is the best guess for which position the player actually played in isolation of anything else. The teamPosition is the best guess for which position the player actually played if we add the constrai32 that each team must have one top player, one jungle, one middle, etc. Generally the recommendation is to use the teamPosition field over the individualPosition field.
    team_position: String,
    time_c_cing_others: i32,
    time_played: i32,
    total_damage_dealt: i32,
    total_damage_dealt_to_champions: i32,
    total_damage_shielded_on_teammates: i32,
    total_damage_taken: i32,
    total_heal: i32,
    total_heals_on_teammates: i32,
    total_minions_killed: i32,
    total_time_c_c_dealt: i32,
    total_time_spent_dead: i32,
    total_units_healed: i32,
    triple_kills: i32,
    true_damage_dealt: i32,
    true_damage_dealt_to_champions: i32,
    true_damage_taken: i32,
    turret_kills: i32,
    turret_takedowns: i32,
    turrets_lost: i32,
    unreal_kills: i32,
    vision_score: i32,
    vision_wards_bought_in_game: i32,
    wards_killed: i32,
    wards_placed: i32,
    win: bool,
}

#[derive(Deserialize, Debug, Clone, PartialEq, Mutable)]
#[serde(rename_all = "camelCase")]
pub struct PerksDto {
    stat_perks: PerkStatsDto,
    styles: Vec<PerkStyleDto>,
}

#[derive(Deserialize, Debug, Clone, PartialEq, Mutable)]
#[serde(rename_all = "camelCase")]
pub struct PerkStatsDto {
    defense: i32,
    flex: i32,
    offense: i32,
}

#[derive(Deserialize, Debug, Clone, PartialEq, Mutable, SoftEq)]
#[serde(rename_all = "camelCase")]
pub struct PerkStyleDto {
    #[softeq(uid)]
    description: String,
    selections: Vec<PerkStyleSelectionDto>,
    style: i32,
}

#[derive(Deserialize, Debug, Clone, PartialEq, Mutable, SoftEq)]
#[serde(rename_all = "camelCase")]
pub struct PerkStyleSelectionDto {
    #[softeq(uid)]
    perk: i32,
    var1: i32,
    var2: i32,
    var3: i32,
}

#[derive(Deserialize, Debug, Clone, PartialEq, Mutable, SoftEq)]
#[serde(rename_all = "camelCase")]
pub struct TeamDto {
    bans: Vec<BanDto>,
    objectives: ObjectivesDto,
    #[softeq(uid)]
    team_id: i32,
    win: bool,
}

#[derive(Deserialize, Debug, Clone, PartialEq, Mutable, SoftEq)]
#[serde(rename_all = "camelCase")]
pub struct BanDto {
    #[softeq(uid)]
    pick_turn: i32,
    champion_id: i32,
}

#[derive(Deserialize, Debug, Clone, PartialEq, Mutable)]
#[serde(rename_all = "camelCase")]
pub struct ObjectivesDto {
    baron: ObjectiveDto,
    champion: ObjectiveDto,
    dragon: ObjectiveDto,
    inhibitor: ObjectiveDto,
    rift_herald: ObjectiveDto,
    tower: ObjectiveDto,
}

#[derive(Deserialize, Debug, Clone, PartialEq, Mutable)]
#[serde(rename_all = "camelCase")]
pub struct ObjectiveDto {
    first: bool,
    kills: i32,
}
