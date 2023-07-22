use super::SkinnyUser;
use crate::util::parsers::parse_iso8601_date;
use chrono::{DateTime, Utc};
use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GroupShout {
    pub body: String,
    pub poster: SkinnyUser,
    #[serde(deserialize_with = "parse_iso8601_date")]
    pub created: DateTime<Utc>,
    #[serde(deserialize_with = "parse_iso8601_date")]
    pub updated: DateTime<Utc>,
}
