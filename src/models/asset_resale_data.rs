use crate::util::parsers::parse_iso8601_date;
use chrono::{DateTime, Utc};
use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct PriceDataPoint {
    pub value: i64,
    #[serde(deserialize_with = "parse_iso8601_date")]
    pub date: DateTime<Utc>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct AssetResaleData {
    pub asset_stock: i64,
    pub sales: i64,
    pub number_remaining: i64,
    pub recent_average_price: i64,
    pub original_price: i64,
    pub price_data_points: Vec<PriceDataPoint>,
}
