use chrono::{DateTime, Utc};
use serde::Deserialize;

use crate::util::parsers::parse_iso8601_date;

use super::Member;

#[derive(Deserialize, Debug, Clone)]
pub struct WallPost {
    pub id: i64,
    pub poster: Option<Member>,
    pub body: String,
    #[serde(deserialize_with = "parse_iso8601_date")]
    pub created: DateTime<Utc>,
    #[serde(deserialize_with = "parse_iso8601_date")]
    pub updated: DateTime<Utc>,
}
