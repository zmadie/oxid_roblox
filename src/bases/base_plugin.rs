use crate::derives::PluginDerive;

#[derive(Debug, Clone)]
pub struct BasePlugin {
    pub id: i64,
}

impl PluginDerive for BasePlugin {
    fn id(&self) -> i64 {
        self.id
    }
}
