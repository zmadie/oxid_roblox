use crate::derives::AssetDerive;

#[derive(Debug, Clone)]
pub struct BaseAsset {
    pub id: i64,
}

impl AssetDerive for BaseAsset {
    fn id(&self) -> i64 {
        self.id
    }
}
