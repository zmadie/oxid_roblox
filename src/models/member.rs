use serde::Deserialize;

use crate::derives::UserDerive;

use super::{GroupRole, SkinnyUser};

#[derive(Deserialize, Debug, Clone)]
pub struct Member {
    pub user: SkinnyUser,
    pub role: GroupRole,
}

impl UserDerive for Member {
    fn id(&self) -> i64 {
        self.user.id
    }
}
