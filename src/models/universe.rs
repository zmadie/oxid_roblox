use std::collections::HashMap;

use chrono::{DateTime, Utc};
use serde::Deserialize;

use crate::{
    derives::UniverseDerive,
    util::parsers::{parse_iso8601_date, parse_universe_creator},
};

use super::CreatorType;

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UniverseLiveStats {
    pub total_player_count: i64,
    pub game_count: i64,
    pub player_counts_by_device_type: HashMap<String, i64>,
}

#[derive(Deserialize, Debug, Clone)]
pub enum UniverseAvatarType {
    MorphToR6,
    MorphToR15,
    PlayerChoice,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Universe {
    pub id: i64,
    pub root_place_id: i64,
    pub name: String,
    pub description: Option<String>,
    pub source_name: String,
    pub source_description: String,

    #[serde(deserialize_with = "parse_universe_creator")]
    pub creator: CreatorType,

    pub price: Option<i64>,
    pub allowed_gear_genres: Vec<String>,
    pub allowed_gear_categories: Vec<String>,
    pub is_genre_enforced: bool,
    pub copying_allowed: bool,
    pub playing: i64,
    pub visits: i64,
    pub max_players: i64,

    #[serde(deserialize_with = "parse_iso8601_date")]
    pub created: DateTime<Utc>,

    #[serde(deserialize_with = "parse_iso8601_date")]
    pub updated: DateTime<Utc>,

    pub studio_access_to_apis_allowed: bool,
    pub create_vip_servers_allowed: bool,
    pub universe_avatar_type: UniverseAvatarType,
    pub genre: String,
    pub is_all_genre: bool,
    // isFavoritedByUser
    pub favorited_count: i64,
}

impl UniverseDerive for Universe {
    fn id(&self) -> i64 {
        self.id
    }
}
