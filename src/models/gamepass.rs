use crate::{bases::BaseUser, util::parsers::parse_optional_base_user};
use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GamePass {
    pub id: i64,
    pub name: String,
    pub display_name: String,
    pub product_id: Option<i64>,
    pub price: Option<i64>,
    pub seller_name: String,
    #[serde(rename = "sellerId")]
    #[serde(deserialize_with = "parse_optional_base_user")]
    pub seller: Option<BaseUser>,
    // is_owned
}
