use serde::Deserialize;

use crate::derives::UniverseDerive;

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SkinnyUniverse {
    pub id: i64,
    pub name: String,
    pub root_place_id: i64,
}

impl UniverseDerive for SkinnyUniverse {
    fn id(&self) -> i64 {
        self.id
    }
}
