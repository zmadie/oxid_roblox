use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GroupRole {
    pub id: i64,
    pub name: String,
    pub description: Option<String>,
    pub rank: u8,
    pub member_count: Option<i32>,
}
