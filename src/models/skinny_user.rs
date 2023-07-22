use crate::derives::UserDerive;
use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SkinnyUser {
    pub has_verified_badge: Option<bool>,
    #[serde(alias = "userId")]
    pub id: i64,
    #[serde(alias = "username")]
    pub name: String,
    pub display_name: Option<String>,
}

impl UserDerive for SkinnyUser {
    fn id(&self) -> i64 {
        self.id
    }
}
