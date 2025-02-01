use serde::{Deserialize, Serialize};
use crate::events::common::LocalisedValue;
use crate::events::station_services::missions::common::MissionEffect;

/// Faction Trend
#[allow(missing_docs)]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub enum FactionTrend {
    UpGood,
    UpBad,
    DownGood,
    DownBad
}

/// A faction effect struct
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(from = "FactionEffectSchema", into = "FactionEffectSchema")]
pub struct FactionEffect {
    /// The detail of the effect
    pub effect: LocalisedValue,

    /// The trend of the event
    pub trend: FactionTrend,
}

impl From<FactionEffectSchema> for FactionEffect {
    /// Convert from FactionEffectSchema
    fn from(value: FactionEffectSchema) -> Self {
        Self {
            effect: LocalisedValue::new(value.effect, Some(value.localised_effect)),
            trend: value.trend,
        }
    }
}

/// Intermediary for localised values
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct FactionEffectSchema {

    /// The detail of the effect
    pub effect: String,

    /// The localised value of the effect,
    #[serde(rename = "Effect_Localised")]
    pub localised_effect: String,

    /// The trend of the event
    pub trend: FactionTrend,
}

impl From<FactionEffect> for FactionEffectSchema {
    /// Convert from FactionEffect
    fn from(value: FactionEffect) -> Self {
        Self {
            effect: value.effect.value,
            localised_effect: value.effect.localised_value.unwrap(),
            trend: value.trend,
        }
    }
}

/// The influence gained/lost to a faction as part of a mission
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct FactionMissionInfluence {
    /// The system address
    pub system_address: u64,
    /// The trend of the influence
    pub trend: FactionTrend,
    /// The influence gain
    #[serde(deserialize_with = "deserialise_mission_effect", serialize_with = "serialise_mission_effect")]
    pub influence: MissionEffect
}

/// A struct to represent faction effect of mission outcome
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct FactionEffectEntry {
    /// The name of the faction
    pub faction: String,

    /// The effects applied to the faction
    pub effects: Vec<FactionEffect>,

    /// The influnce affected for the faction
    pub influence: Vec<FactionMissionInfluence>,

    /// Reputation trend
    pub reputation_trend: FactionTrend,

    /// Reputation effectg
    #[serde(deserialize_with = "deserialise_mission_effect", serialize_with = "serialise_mission_effect")]
    pub reputation: MissionEffect,
}


/// A commodity reward from a mission
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(from = "CommodityRewardSchema", into = "CommodityRewardSchema")]
pub struct CommodityReward {
    /// The name of the commodity
    pub name: LocalisedValue,

    /// Number rewarded
    pub count: u64,
}

impl From<CommodityRewardSchema> for CommodityReward {
    /// Convert from Commodity reward schema
    fn from(value: CommodityRewardSchema) -> Self {
        Self {
            name: LocalisedValue::new(value.name, Some(value.localised_name)),
            count: value.count,}
    }
}

/// Intermediary struct due to localised value
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct CommodityRewardSchema {
    /// The name of the commodity
    pub name: String,

    /// Localised name of the commodity
    #[serde(rename = "Name_Localised")]
    pub localised_name: String,

    /// Number rewarded
    pub count: u64,
}

impl From<CommodityReward> for CommodityRewardSchema {
    /// Convert from Commodity Reward
    fn from(value: CommodityReward) -> Self {
        Self {
            name: value.name.value,
            localised_name: value.name.localised_value.unwrap(),
            count: value.count,}
    }
}

/// A material reward from a mission
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(from = "MaterialRewardSchema", into = "MaterialRewardSchema")]
pub struct MaterialReward {
    /// The name of the material
    pub name: LocalisedValue,
    /// Category of the material
    pub category: LocalisedValue,
    /// Number
    pub count: u64,
}

impl From<MaterialRewardSchema> for MaterialReward {
    /// Convert from Scheam to event struct
    fn from(schema: MaterialRewardSchema) -> Self {
        Self {
            name: LocalisedValue::new(schema.name, Some(schema.localised_name)),
            category: LocalisedValue::new(schema.category, Some(schema.localised_category)),
            count: schema.count,
        }
    }
}

/// As the material reward contains a localised value we need an intermediary object
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct MaterialRewardSchema {
    /// The name of the material
    pub name: String,
    /// Localised name of the material
    #[serde(rename = "Name_Localised")]
    pub localised_name: String,
    /// Category of the material
    pub category: String,
    /// Localised category
    #[serde(rename = "Category_Localised")]
    pub localised_category: String,
    /// Number
    pub count: u64,
}

impl From<MaterialReward> for MaterialRewardSchema {
    /// Convert from material reward to schema
    fn from(value: MaterialReward) -> Self {
        Self {
            name: value.name.value,
            localised_name: value.name.localised_value.unwrap(),
            category: value.category.value,
            localised_category: value.category.localised_value.unwrap(),
            count: value.count,
        }
    }
}

/// This function will serialise the mission effect enum to the relevant textual value '+' to '+++++'
fn serialise_mission_effect<S>(value: &MissionEffect, serializer: S) -> Result<S::Ok, S::Error>
where S: serde::Serializer {
    value.to_string().serialize(serializer)
}

/// Deserialise mission effect enum from textual value, '+' to '+++++'
fn deserialise_mission_effect<'de, D>(deserialiser: D) -> Result<MissionEffect, D::Error>
where D: serde::Deserializer<'de> {
    let value = String::deserialize(deserialiser)?;

    Ok(MissionEffect::from(value))
}