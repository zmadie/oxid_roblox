use async_trait::async_trait;

use crate::{
    models::{Badge, GamePass, SocialLink, UniverseLiveStats},
    util::{
        api_helper,
        paging::{identity_mapper, PageIterator},
        responses::{ApiArrayResponse, UniverseFavoriteCountResponse},
        ResultExtensions, RobloxResult,
    },
};

#[async_trait(?Send)]
pub trait Universe {
    #[doc(hidden)]
    fn id(&self) -> i64;

    async fn favorite_count(&self) -> RobloxResult<i64> {
        api_helper::get(format!(
            "https://games.roblox.com/v1/games/{}/favorites/count",
            self.id()
        ))
        .await
        .map_async(api_helper::deserialize_body::<UniverseFavoriteCountResponse>)
        .await
        .map(|data| data.favorites_count)
    }

    fn badges(&self) -> PageIterator<Badge, Badge> {
        PageIterator::new(
            format!(
                "https://badges.roblox.com/v1/universes/{}/badges",
                self.id()
            ),
            identity_mapper,
        )
    }

    async fn live_stats(&self) -> RobloxResult<UniverseLiveStats> {
        api_helper::get(format!(
            "https://develop.roblox.com/v1/universes/{}/live-stats",
            self.id()
        ))
        .await
        .map_async(api_helper::deserialize_body)
        .await
    }

    fn gamepasses(&self) -> PageIterator<GamePass, GamePass> {
        PageIterator::new(
            format!(
                "https://games.roblox.com/v1/games/{}/game-passes",
                self.id()
            ),
            identity_mapper,
        )
    }

    async fn social_links(&self) -> RobloxResult<Vec<SocialLink>> {
        api_helper::get(format!(
            "https://games.roblox.com/v1/games/{}/social-links/list",
            self.id()
        ))
        .await
        .map_async(api_helper::deserialize_body::<ApiArrayResponse<SocialLink>>)
        .await
        .map(|data| data.data)
    }
}
