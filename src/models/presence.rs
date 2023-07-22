use crate::{
    bases::BaseUniverse,
    util::{
        api_helper, parsers::parse_optional_base_universe, responses::PresenceLastOnlineResponse,
        ResultExtensions, RobloxResult,
    },
};
use chrono::{DateTime, Utc};
use serde::Deserialize;
use serde_json::json;
use serde_repr::Deserialize_repr;

#[derive(Deserialize_repr, Debug, Clone)]
#[repr(i32)]
pub enum PresenceType {
    Offline,
    Online,
    InGame,
    InStudio,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Presence {
    pub user_presence_type: PresenceType,
    pub last_location: String,
    pub place_id: Option<i64>,
    pub root_place_id: Option<i64>,
    pub game_id: Option<String>,
    #[serde(rename = "universeId")]
    #[serde(deserialize_with = "parse_optional_base_universe")]
    pub universe: Option<BaseUniverse>,
    pub user_id: i64,
    // invisibleModeExpiry
}

impl Presence {
    pub async fn last_online(&self) -> RobloxResult<DateTime<Utc>> {
        api_helper::post(
            "https://presence.roblox.com/v1/presence/last-online".to_owned(),
            json!({
                "userIds": [self.user_id]
            }),
        )
        .await
        .map_async(api_helper::deserialize_body::<PresenceLastOnlineResponse>)
        .await
        .map(|data| {
            data.last_online_timestamps
                .first()
                .cloned()
                .unwrap()
                .last_online
        })
    }
}
