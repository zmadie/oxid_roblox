use serde::Deserialize;

use crate::{bases::BaseUniverse, util::parsers::parse_base_universe};

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Place {
    // "placeId": 142823291,
    // "name": "Murder Mystery 2",
    // "description": "Can you solve the Mystery and survive each round?\r\n\r\nINNOCENTS: Run and hide from the Murderer. Use your detective skills to expose the Murderer.\r\nSHERIFF: Work with the Innocents; you are the only one with a weapon who can take down the Murderer. \r\nMURDERER: Eliminate EVERYONE. Don't get shot by the sheriff.\r\n\r\nCollect and trade hundreds of knives! \r\n\r\nCheck out my profile for quick links to my pages! \r\nLike, follow, and subscribe for exclusive update news, free knife codes, and more!",
    // "sourceName": "Murder Mystery 2",
    // "sourceDescription": "Can you solve the Mystery and survive each round?\r\n\r\nINNOCENTS: Run and hide from the Murderer. Use your detective skills to expose the Murderer.\r\nSHERIFF: Work with the Innocents; you are the only one with a weapon who can take down the Murderer. \r\nMURDERER: Eliminate EVERYONE. Don't get shot by the sheriff.\r\n\r\nCollect and trade hundreds of knives! \r\n\r\nCheck out my profile for quick links to my pages! \r\nLike, follow, and subscribe for exclusive update news, free knife codes, and more!",
    // "url": "https://www.roblox.com/games/142823291/Murder-Mystery-2",
    // "builder": "Nikilis",
    // "builderId": 1848960,
    // "hasVerifiedBadge": false,
    // "isPlayable": true,
    // "reasonProhibited": "None",
    // "universeId": 66654135,
    // "universeRootPlaceId": 142823291,
    // "price": 0,
    // "imageToken": "T_142823291_87f0"
    #[serde(rename = "placeId")]
    pub id: i64,
    pub name: String,
    pub description: String,
    pub source_name: String,
    pub source_description: String,
    pub url: String,
    #[serde(rename = "builder")]
    pub builder_username: String,
    pub builder_id: i64,
    pub has_verified_badge: bool,
    pub is_playable: bool,
    pub reason_prohibited: String,
    #[serde(rename = "universeId")]
    #[serde(deserialize_with = "parse_base_universe")]
    pub universe: BaseUniverse,
    pub universe_root_place_id: i64,
    pub price: i64,
    pub image_token: String,
}
