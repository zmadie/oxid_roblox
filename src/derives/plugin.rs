use async_trait::async_trait;
use serde_json::json;

use crate::util::{api_helper, RobloxResult};

#[async_trait(?Send)]
pub trait Plugin {
    #[doc(hidden)]
    fn id(&self) -> i64;

    async fn update(
        &self,
        name: Option<&str>,
        description: Option<&str>,
        comments_enabled: Option<bool>,
    ) -> RobloxResult<()> {
        api_helper::patch(
            format!("https://develop.roblox.com/v1/plugins/{}", self.id()),
            json!({
                "name": name,
                "description": description,
                "commentsEnabled": comments_enabled,
            }),
        )
        .await
        .map(|_| ())
    }
}
