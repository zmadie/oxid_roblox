use chrono::{DateTime, Utc};
use serde::Deserialize;

use crate::models::{GroupRole, Presence};

use super::{parsers::parse_iso8601_date, ApiError};

#[derive(Deserialize)]
pub struct ErrorResponse {
    pub errors: Vec<ApiError>,
}

#[derive(Deserialize)]
pub struct ApiArrayResponse<T> {
    pub data: Vec<T>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserPresencesResponse {
    pub user_presences: Vec<Presence>,
}

#[derive(Deserialize)]
pub struct GroupRolesResponse {
    pub roles: Vec<GroupRole>,
}

#[derive(Deserialize)]
pub struct CurrencyResponse {
    pub robux: i64,
}

#[derive(Deserialize)]
pub struct CountResponse {
    pub count: i32,
}

#[derive(Deserialize)]
pub struct UsernameHistoryResponse {
    pub name: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UniverseFavoriteCountResponse {
    pub favorites_count: i64,
}

#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct AssetCreatorResponse {
    pub id: i64,
    pub name: String,
    pub creator_type: String,
    pub creator_target_id: i64,
    pub has_verified_badge: bool,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UniverseCreatorResponse {
    pub id: i64,
    pub name: String,
    #[serde(rename = "type")]
    pub creator_type: String,
    pub has_verified_badge: bool,
}

#[derive(Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PresenceLastOnline {
    pub user_id: i64,
    #[serde(deserialize_with = "parse_iso8601_date")]
    pub last_online: DateTime<Utc>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PresenceLastOnlineResponse {
    pub last_online_timestamps: Vec<PresenceLastOnline>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PageResponse<T> {
    pub previous_page_cursor: Option<String>,
    pub next_page_cursor: Option<String>,
    pub data: Vec<T>,
}
