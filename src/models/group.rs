use crate::{derives::GroupDerive, util::RobloxResult};
use async_trait::async_trait;
use serde::Deserialize;

use super::{GroupShout, SkinnyUser};

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Group {
    pub id: i64,
    pub name: String,
    pub description: String,
    pub owner: Option<SkinnyUser>,
    pub shout: Option<GroupShout>,
    pub member_count: i32,
    pub is_builders_club_only: bool,
    pub public_entry_allowed: bool,
    #[serde(default)] // Roblox doesn't return an isLocked field for groups that are not locked
    pub is_locked: bool,
    pub has_verified_badge: bool,
}

#[async_trait(?Send)]
impl GroupDerive for Group {
    fn id(&self) -> i64 {
        self.id
    }

    /// A version of [`GroupDerive::update_shout`] that updates the shout field of this group.
    async fn update_shout(&mut self, message: String) -> RobloxResult<GroupShout> {
        let shout = self.update_shout(message).await?;
        self.shout = Some(shout.clone());
        Ok(shout)
    }
}
