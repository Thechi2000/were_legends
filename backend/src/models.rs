//! Module containing all the structures that can be deserialized from the `https://127.0.0.1:2999/liveclientdata/` endpoint.
use mutable::{cmp::SoftEq, Mutable, SoftEq};
use serde::Deserialize;

#[derive(Deserialize, Debug, Clone, PartialEq, Mutable)]
#[serde(rename_all = "camelCase")]
pub struct AllGameData {
    pub active_player: ActivePlayer,
    pub all_players: Vec<PlayerData>,
    pub events: Events,
    pub game_data: GameData,
}

#[derive(Deserialize, Debug, Clone, PartialEq, Mutable)]
#[serde(rename_all = "camelCase")]
pub struct ActivePlayer {
    pub abilities: Abilities,
    pub champion_stats: ChampionStats,
    pub current_gold: f64,
    pub full_runes: FullRunes,
    pub level: usize,
    pub summoner_name: String,
}

#[derive(Deserialize, Debug, Clone, PartialEq, Mutable, SoftEq)]
#[serde(rename_all = "camelCase")]
pub struct PlayerData {
    pub champion_name: String,
    pub is_bot: bool,
    pub is_dead: bool,
    pub items: Vec<Item>,
    pub level: usize,
    pub position: String, // TODO: Enum
    pub raw_champion_name: String,
    pub respawn_timer: f64,
    pub runes: PartialRunes,
    pub scores: Scores,
    #[serde(rename = "skinID")]
    pub skin_id: usize,
    #[softeq(uid)]
    pub summoner_name: String,
    pub summoner_spells: SummonerSpells,
    pub team: Team,
}

// TODO
#[derive(Deserialize, Debug, Clone, PartialEq, Mutable, SoftEq)]
#[serde(rename_all = "camelCase")]
pub struct Item {
    // TODO
    #[softeq(uid)]
    pub item_id: usize
}

#[derive(Deserialize, Debug, Clone, PartialEq, Mutable)]
#[serde(rename_all = "camelCase")]
pub struct Scores {
    pub assists: usize,
    pub creep_score: usize,
    pub deaths: usize,
    pub kills: usize,
    pub ward_score: f64, // TODO: why not usize ?
}

#[derive(Deserialize, Debug, Clone, PartialEq, Mutable)]
#[serde(rename_all = "camelCase")]
pub struct SummonerSpells {
    pub summoner_spell_one: SummonerSpell,
    pub summoner_spell_two: SummonerSpell,
}

#[derive(Deserialize, Debug, Clone, PartialEq, Mutable)]
#[serde(rename_all = "camelCase")]
pub struct SummonerSpell {
    pub display_name: String,
    pub raw_description: String,
    pub raw_display_name: String,
}

#[derive(Deserialize, Debug, Clone, PartialEq, Mutable)]
#[serde(rename_all = "PascalCase")]
pub struct Abilities {
    pub passive: Ability,
    pub q: Ability,
    pub w: Ability,
    pub e: Ability,
    pub r: Ability,
}

#[derive(Deserialize, Debug, Clone, PartialEq, Mutable)]
#[serde(rename_all = "camelCase")]
pub struct Ability {
    pub ability_level: Option<u8>,
    pub display_name: String,
    pub id: String,
    pub raw_description: String,
    pub raw_display_name: String,
}

#[derive(Deserialize, Debug, Clone, PartialEq, Mutable)]
#[serde(rename_all = "camelCase")]
pub struct ChampionStats {
    pub ability_power: f64,
    pub armor: f64,
    pub armor_penetration_flat: f64,
    pub attack_damage: f64,
    pub attack_range: f64,
    pub attack_speed: f64,
    pub bonus_armor_penetration_percent: f64,
    pub bonus_magic_penetration_percent: f64,
    pub crit_chance: f64,
    pub crit_damage: f64,
    pub current_health: f64,
    pub heal_shield_power: Option<f64>,
    pub health_regen_rate: f64,
    pub life_steal: f64,
    pub magic_lethality: f64,
    pub magic_penetration_flat: f64,
    pub magic_penetration_percent: f64,
    pub magic_resist: f64,
    pub max_health: f64,
    pub move_speed: f64,
    pub omnivamp: Option<f64>,
    pub physical_lethality: f64,
    pub physical_vamp: Option<f64>,
    pub resource_max: f64,
    pub resource_regen_rate: f64,
    pub resource_type: String, // TODO: Enum
    pub resource_value: f64,
    pub spell_vamp: f64,
    pub tenacity: f64,
}

/// Runes for the active player
#[derive(Deserialize, Debug, Clone, PartialEq, Mutable)]
#[serde(rename_all = "camelCase")]
pub struct FullRunes {
    pub general_runes: Vec<Rune>,
    pub keystone: Rune,
    pub primary_rune_tree: RuneTree,
    pub secondary_rune_tree: RuneTree,
    pub stat_runes: Vec<StatRunes>,
}

/// Runes for all the other players
#[derive(Deserialize, Debug, Clone, PartialEq, Mutable)]
#[serde(rename_all = "camelCase")]
pub struct PartialRunes {
    pub keystone: Rune,
    pub primary_rune_tree: RuneTree,
    pub secondary_rune_tree: RuneTree,
}

#[derive(Deserialize, Debug, Clone, PartialEq, Mutable, SoftEq)]
#[serde(rename_all = "camelCase")]
pub struct Rune {
    #[softeq(uid)]
    pub id: u16,
    pub display_name: String,
    pub raw_description: String,
    pub raw_display_name: String,
}

#[derive(Deserialize, Debug, Clone, PartialEq, Mutable)]
#[serde(rename_all = "camelCase")]
pub struct RuneTree {
    pub id: u16,
    pub display_name: String,
    pub raw_description: String,
    pub raw_display_name: String,
}

#[derive(Deserialize, Debug, Clone, PartialEq, Mutable, SoftEq)]
#[serde(rename_all = "camelCase")]
pub struct StatRunes {
    #[softeq(uid)]
    pub id: u16,
    pub raw_description: String,
}

#[derive(Deserialize, Debug, Clone, PartialEq, Mutable)]
pub struct GameData {
    pub game_mode: GameMode,
    pub game_time: f64,
    pub map_name: String,
    pub map_number: usize,
    pub map_terrain: String, // TODO: Enum
}

#[derive(Deserialize, Debug, Clone, PartialEq, SoftEq, Mutable)]
#[serde(rename_all = "PascalCase")]
pub struct Event {
    #[softeq(uid)]
    pub event_id: usize,
    pub event_time: f64,
    pub data: EventData,
}

#[derive(Deserialize, Debug, Clone, PartialEq, Mutable)]
#[serde(rename_all = "PascalCase")]
pub struct Events {
    pub events: Vec<Event>,
}

#[derive(Deserialize, Debug, Clone, PartialEq, Mutable)]
#[serde(tag = "EventName", rename_all = "PascalCase")]
pub enum EventData {
    GameStart,
    GameEnd {
        result: GameResult,
    },
    MinionsSpawning,
    FirstBrick {
        // TODO: what is it ?
        killer_name: String,
    },
    FirstBlood {
        recipient: String, // TODO: what is it ?
    },
    TurretKilled {
        killer_name: String,
        turret_killed: String, // TODO: what is it ?
        assisters: Vec<String>,
    },
    InhibKilled {
        killer_name: String,
        inhib_killed: String, // TODO: what is it ?
        assisters: Vec<String>,
    },
    InhibRespawningSoon {
        inhib_respawning_soon: String, // TODO: what is it ?
    },
    InhibRespawned {
        inhib_respawned: String, // TODO: what is it ?
    },
    DragonKill {
        killer_name: String,
        assisters: Vec<String>,
        dragon_type: DragonType,
        stolen: String, // TODO: bool
    },
    HeraldKill {
        killer_name: String,
        assisters: Vec<String>,
        stolen: String, // TODO: bool
    },
    BaronKill {
        killer_name: String,
        assisters: Vec<String>,
        stolen: String, // TODO: bool
    },
    ChampionKill {
        killer_name: String,
        victim_name: String,
        assisters: Vec<String>,
    },
    Multikill {
        killer_name: String,
        kill_streak: u8,
    },
    Ace {
        acer: String,
        acing_team: Team,
    },
}

#[derive(Deserialize, Debug, Clone, PartialEq, Mutable)]
pub enum GameResult {
    Win,
    Loss,
}

#[derive(Deserialize, Debug, Clone, PartialEq, Mutable)]
pub enum DragonType {
    Elder,
    Earth,
    Air,
    Fire,
    Water,
    Hextech,
    Chemtech,
}

#[derive(Deserialize, Debug, Clone, PartialEq, Mutable)]
#[serde(rename_all = "UPPERCASE")]
pub enum GameMode {
    Classic,
    Aram,
}

#[derive(Deserialize, Debug, Clone, PartialEq, Mutable)]
#[serde(rename_all = "UPPERCASE")]
pub enum Team {
    Order,
    Chaos,
}
