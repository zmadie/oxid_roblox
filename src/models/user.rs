use crate::{derives::UserDerive, util::parsers::parse_iso8601_date};
use chrono::{DateTime, Utc};
use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub description: String,
    #[serde(deserialize_with = "parse_iso8601_date")]
    pub created: DateTime<Utc>,
    pub is_banned: bool,
    pub external_app_display_name: Option<String>,
    pub has_verified_badge: bool,
    pub id: i64,
    pub name: String,
    pub display_name: String,
}

impl UserDerive for User {
    fn id(&self) -> i64 {
        self.id
    }
}
