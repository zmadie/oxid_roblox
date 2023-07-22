use chrono::{DateTime, Utc};
use serde::Deserialize;

use crate::{
    bases::BaseAsset,
    util::parsers::{parse_base_asset, parse_iso8601_date},
};

use super::SkinnyUniverse;

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct BadgeStatistics {
    pub past_day_awarded_count: i64,
    pub awarded_count: i64,
    pub win_rate_percentage: f64,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Badge {
    pub id: i64,
    pub name: String,
    pub description: Option<String>,
    pub display_name: String,
    pub display_description: Option<String>,
    pub enabled: bool,
    #[serde(rename = "iconImageId")]
    #[serde(deserialize_with = "parse_base_asset")]
    pub icon: BaseAsset,
    #[serde(rename = "displayIconImageId")]
    #[serde(deserialize_with = "parse_base_asset")]
    pub display_icon: BaseAsset,
    #[serde(deserialize_with = "parse_iso8601_date")]
    pub created: DateTime<Utc>,
    #[serde(deserialize_with = "parse_iso8601_date")]
    pub updated: DateTime<Utc>,
    pub statistics: BadgeStatistics,
    pub awarding_universe: SkinnyUniverse,
}
