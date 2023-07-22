use chrono::{DateTime, Utc};
use serde::Deserialize;

use super::SkinnyUser;
use crate::util::parsers::parse_iso8601_date;

#[derive(Deserialize, Debug, Clone)]
pub struct JoinRequest {
    pub requester: SkinnyUser,
    #[serde(deserialize_with = "parse_iso8601_date")]
    pub created: DateTime<Utc>,
}
