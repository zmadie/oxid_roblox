use std::collections::HashMap;

use chrono::{DateTime, Utc};
use lazy_static::lazy_static;
use serde::{de::Error, Deserialize, Deserializer};

use crate::{
    bases::{BaseAsset, BaseUniverse, BaseUser},
    models::{AssetType, CreatorType, SkinnyGroup, SkinnyUser},
};

use super::responses::{AssetCreatorResponse, UniverseCreatorResponse};

pub fn parse_iso8601_date<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
where
    D: Deserializer<'de>,
{
    Deserialize::deserialize(deserializer)
        .and_then(|date_string: &str| date_string.parse::<DateTime<Utc>>().map_err(Error::custom))
}

pub fn parse_base_asset<'de, D>(deserializer: D) -> Result<BaseAsset, D::Error>
where
    D: Deserializer<'de>,
{
    Deserialize::deserialize(deserializer).map(|id: i64| BaseAsset { id })
}

pub fn parse_base_universe<'de, D>(deserializer: D) -> Result<BaseUniverse, D::Error>
where
    D: Deserializer<'de>,
{
    Deserialize::deserialize(deserializer).map(|id: i64| BaseUniverse { id })
}

pub fn parse_optional_base_user<'de, D>(deserializer: D) -> Result<Option<BaseUser>, D::Error>
where
    D: Deserializer<'de>,
{
    Deserialize::deserialize(deserializer).map(|id: Option<i64>| id.map(|id| BaseUser { id }))
}

pub fn parse_optional_base_universe<'de, D>(
    deserializer: D,
) -> Result<Option<BaseUniverse>, D::Error>
where
    D: Deserializer<'de>,
{
    Deserialize::deserialize(deserializer).map(|id: Option<i64>| id.map(|id| BaseUniverse { id }))
}

pub fn parse_asset_creator<'de, D>(deserializer: D) -> Result<CreatorType, D::Error>
where
    D: Deserializer<'de>,
{
    Deserialize::deserialize(deserializer).map(|creator: AssetCreatorResponse| {
        if creator.creator_type == "User" {
            CreatorType::User(SkinnyUser {
                has_verified_badge: None,
                id: creator.creator_target_id,
                name: creator.name,
                display_name: None,
            })
        } else {
            CreatorType::Group(SkinnyGroup {
                name: creator.name,
                id: creator.creator_target_id,
                has_verified_badge: creator.has_verified_badge,
            })
        }
    })
}

pub fn parse_universe_creator<'de, D>(deserializer: D) -> Result<CreatorType, D::Error>
where
    D: Deserializer<'de>,
{
    Deserialize::deserialize(deserializer).map(|creator: UniverseCreatorResponse| {
        if creator.creator_type == "User" {
            CreatorType::User(SkinnyUser {
                has_verified_badge: Some(creator.has_verified_badge),
                id: creator.id,
                name: creator.name,
                display_name: None,
            })
        } else {
            CreatorType::Group(SkinnyGroup {
                id: creator.id,
                name: creator.name,
                has_verified_badge: creator.has_verified_badge,
            })
        }
    })
}

pub fn parse_asset_type<'de, D>(deserializer: D) -> Result<AssetType, D::Error>
where
    D: Deserializer<'de>,
{
    Deserialize::deserialize(deserializer).map(|id| AssetType {
        id,
        name: ASSET_TYPE_NAMES.get(&id).cloned(),
    })
}

lazy_static! {
    static ref ASSET_TYPE_NAMES: HashMap<u8, String> = {
        let mut map = HashMap::new();
        map.insert(1, "Image".to_string());
        map.insert(2, "T-Shirt".to_string());
        map.insert(3, "Audio".to_string());
        map.insert(4, "Mesh".to_string());
        map.insert(5, "Lua".to_string());
        map.insert(6, "HTML".to_string());
        map.insert(7, "Text".to_string());
        map.insert(8, "Hat".to_string());
        map.insert(9, "Place".to_string());
        map.insert(10, "Model".to_string());
        map.insert(11, "Shirt".to_string());
        map.insert(12, "Pants".to_string());
        map.insert(13, "Decal".to_string());
        map.insert(16, "Avatar".to_string());
        map.insert(17, "Head".to_string());
        map.insert(18, "Face".to_string());
        map.insert(19, "Gear".to_string());
        map.insert(21, "Badge".to_string());
        map.insert(22, "Group Emblem".to_string());
        map.insert(24, "Animation".to_string());
        map.insert(25, "Arms".to_string());
        map.insert(26, "Legs".to_string());
        map.insert(27, "Torso".to_string());
        map.insert(28, "Right Arm".to_string());
        map.insert(29, "Left Arm".to_string());
        map.insert(30, "Left Leg".to_string());
        map.insert(31, "Right Leg".to_string());
        map.insert(32, "Package".to_string());
        map.insert(33, "YouTubeVideo".to_string());
        map.insert(34, "Pass".to_string());
        map.insert(35, "App".to_string());
        map.insert(37, "Code".to_string());
        map.insert(38, "Plugin".to_string());
        map.insert(39, "SolidModel".to_string());
        map.insert(40, "MeshPart".to_string());
        map.insert(41, "Hair Accessory".to_string());
        map.insert(42, "Face Accessory".to_string());
        map.insert(43, "Neck Accessory".to_string());
        map.insert(44, "Shoulder Accessory".to_string());
        map.insert(45, "Front Accessory".to_string());
        map.insert(46, "Back Accessory".to_string());
        map.insert(47, "Waist Accessory".to_string());
        map.insert(48, "Climb Animation".to_string());
        map.insert(49, "Death Animation".to_string());
        map.insert(50, "Fall Animation".to_string());
        map.insert(51, "Idle Animation".to_string());
        map.insert(52, "Jump Animation".to_string());
        map.insert(53, "Run Animation".to_string());
        map.insert(54, "Swim Animation".to_string());
        map.insert(55, "Walk Animation".to_string());
        map.insert(56, "Pose Animation".to_string());
        map.insert(59, "LocalizationTableManifest".to_string());
        map.insert(60, "LocalizationTableTranslation".to_string());
        map.insert(61, "Emote Animation".to_string());
        map.insert(62, "Video".to_string());
        map.insert(63, "TexturePack".to_string());
        map.insert(64, "T-Shirt Accessory".to_string());
        map.insert(65, "Shirt Accessory".to_string());
        map.insert(66, "Pants Accessory".to_string());
        map.insert(67, "Jacket Accessory".to_string());
        map.insert(68, "Sweater Accessory".to_string());
        map.insert(69, "Shorts Accessory".to_string());
        map.insert(70, "Left Shoe Accessory".to_string());
        map.insert(71, "Right Shoe Accessory".to_string());
        map.insert(72, "Dress Skirt Accessory".to_string());
        map.insert(73, "Font Family".to_string());
        map.insert(74, "Font Face".to_string());
        map.insert(75, "MeshHiddenSurfaceRemoval".to_string());

        map
    };
}
