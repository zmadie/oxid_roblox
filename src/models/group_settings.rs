use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct GroupSettings {
    pub is_approval_required: bool,
    pub is_builders_club_required: bool,
    pub are_enemies_allowed: bool,
    pub are_group_funds_visible: bool,
    pub are_group_games_visible: bool,
    pub is_group_name_change_enabled: bool,
    pub can_change_group_name: bool,
}
