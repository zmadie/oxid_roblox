use crate::derives::UserDerive;

#[derive(Debug, Clone)]
pub struct BaseUser {
    pub id: i64,
}

impl UserDerive for BaseUser {
    fn id(&self) -> i64 {
        self.id
    }
}
