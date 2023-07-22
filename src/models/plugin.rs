use crate::{derives::PluginDerive, util::parsers::parse_iso8601_date};
use chrono::{DateTime, Utc};
use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Plugin {
    pub id: i64,
    pub name: String,
    pub description: String,
    pub comments_enabled: bool,
    pub version_id: i64,
    #[serde(deserialize_with = "parse_iso8601_date")]
    pub created: DateTime<Utc>,
    #[serde(deserialize_with = "parse_iso8601_date")]
    pub updated: DateTime<Utc>,
}

impl PluginDerive for Plugin {
    fn id(&self) -> i64 {
        self.id
    }
}
