use serde::Deserialize;

use crate::derives::UserDerive;

#[derive(Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PreviousUsernamesSkinnyUser {
    pub previous_usernames: Vec<String>,
    pub has_verified_badge: bool,
    pub id: i64,
    pub name: String,
    pub display_name: String,
}

impl UserDerive for PreviousUsernamesSkinnyUser {
    fn id(&self) -> i64 {
        self.id
    }
}
