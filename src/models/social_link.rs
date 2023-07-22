use serde::Deserialize;

#[derive(Deserialize, Clone, Debug)]
pub enum SocialLinkType {
    Facebook,
    Twitter,
    YouTube,
    Twitch,
    GooglePlus,
    Discord,
    RobloxGroup,
    Amazon,
    Guilded,
}

#[derive(Deserialize, Clone, Debug)]
pub struct SocialLink {
    pub id: i64,
    pub title: String,
    pub url: String,
    #[serde(rename = "type")]
    pub link_type: SocialLinkType,
}
