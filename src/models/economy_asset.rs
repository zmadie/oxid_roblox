use crate::{
    bases::BaseAsset,
    derives::AssetDerive,
    util::parsers::{parse_asset_creator, parse_asset_type, parse_base_asset, parse_iso8601_date},
};
use chrono::{DateTime, Utc};
use serde::Deserialize;

use super::CreatorType;

#[derive(Debug, Clone)]
pub struct AssetType {
    pub id: u8,
    pub name: Option<String>,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct EconomyAsset {
    pub product_type: String,

    #[serde(rename = "AssetId")]
    pub id: i64,

    pub product_id: i64,
    pub name: String,
    pub description: String,

    #[serde(rename = "AssetTypeId")]
    #[serde(deserialize_with = "parse_asset_type")]
    pub asset_type: AssetType,

    #[serde(deserialize_with = "parse_asset_creator")]
    pub creator: CreatorType,

    #[serde(rename = "IconImageAssetId")]
    #[serde(deserialize_with = "parse_base_asset")]
    pub icon_image: BaseAsset,

    #[serde(deserialize_with = "parse_iso8601_date")]
    pub created: DateTime<Utc>,

    #[serde(deserialize_with = "parse_iso8601_date")]
    pub updated: DateTime<Utc>,

    #[serde(rename = "PriceInRobux")]
    pub price: Option<i64>,

    pub sales: i64,
    pub is_new: bool,
    pub is_for_sale: bool,
    pub is_public_domain: bool,
    pub is_limited: bool,
    pub is_limited_unique: bool,
    pub remaining: Option<i64>,
    pub minimum_membership_level: i64,
    pub content_rating_type_id: i64,
    // TODO: update this when there is more information about these things
    // SaleAvailabilityLocations: null
    // SaleLocation:	          null
    // CollectibleItemId:	      null
    // CollectibleProductId:	  null
    // CollectiblesItemDetails:	  null
}

impl AssetDerive for EconomyAsset {
    fn id(&self) -> i64 {
        self.id
    }
}
