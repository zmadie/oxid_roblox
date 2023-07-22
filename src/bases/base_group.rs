use crate::derives::GroupDerive;

#[derive(Debug, Clone)]
pub struct BaseGroup {
    pub id: i64,
}

impl GroupDerive for BaseGroup {
    fn id(&self) -> i64 {
        self.id
    }
}
