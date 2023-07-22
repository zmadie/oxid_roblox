use crate::derives::UniverseDerive;

#[derive(Debug, Clone)]
pub struct BaseUniverse {
    pub id: i64,
}

impl UniverseDerive for BaseUniverse {
    fn id(&self) -> i64 {
        self.id
    }
}
