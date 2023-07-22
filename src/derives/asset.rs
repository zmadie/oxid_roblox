use async_trait::async_trait;

use crate::{
    models::AssetResaleData,
    util::{api_helper, ResultExtensions, RobloxResult},
};

#[async_trait(?Send)]
pub trait Asset {
    #[doc(hidden)]
    fn id(&self) -> i64;
    async fn resale_data(&self) -> RobloxResult<AssetResaleData> {
        api_helper::get(format!(
            "https://economy.roblox.com/v1/assets/{}/resale-data",
            self.id()
        ))
        .await
        .map_async(api_helper::deserialize_body)
        .await
    }
}
