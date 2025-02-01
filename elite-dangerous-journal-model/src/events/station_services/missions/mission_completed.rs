/// Module that contains components for the mission completed event due to the complexity
pub mod components;

use serde::{Deserialize, Serialize};
use crate::events::common::{deconstruct_localised_value, deconstruct_optional_localised_value, EventMeta, LocalisedValue};
use crate::events::station_services::missions::mission_completed::components::{CommodityReward, FactionEffectEntry, MaterialReward};

/// Mission completed event
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(from = "MissionCompletedEventSchema", into = "MissionCompletedEventSchema")]
pub struct MissionCompletedEvent {
    /// Event meta data
    pub event_meta: EventMeta,

    /// The faction of the mission giver
    pub faction: String,

    /// The name of the mission
    pub name: LocalisedValue,

    /// The id of the mission
    pub mission_id: u64,

    /// The commodity involved 
    pub commodity: Option<LocalisedValue>,
    
    /// The commodity count
    pub count: Option<u64>,
    
    /// The target
    pub target: Option<String>,
    
    /// The target type
    pub target_type: Option<LocalisedValue>,
    
    /// The target faction
    pub target_faction: Option<String>,
    
    /// The destination system
    pub destination_system: Option<String>,

    /// The destination station
    pub destination_station: Option<String>,

    /// The destination settlement
    pub destination_settlement: Option<String>,

    /// The reward
    pub reward: Option<u64>,

    /// The donation
    pub donation: Option<String>,

    /// The donated amount
    pub donated: Option<u64>,

    /// Permits awarded
    pub permits_awarded: Option<Vec<String>>,

    /// Material rewards from the mission
    pub materials_reward: Option<Vec<MaterialReward>>,

    /// Comodity rewards from the mission
    pub commodity_reward: Option<Vec<CommodityReward>>,

    /// Faction effects
    pub faction_effects: Option<Vec<FactionEffectEntry>>,
}

impl From<MissionCompletedEventSchema> for MissionCompletedEvent {
    /// Convert from Schema event
    fn from(value: MissionCompletedEventSchema) -> Self {
        Self {
            event_meta: value.event_meta,
            faction: value.faction,
            name: LocalisedValue::new(value.name, Some(value.localised_name)),
            mission_id: value.mission_id,
            commodity: LocalisedValue::new_optional(value.commodity, value.localised_commodity),
            count: value.count,
            target: value.target,
            target_type: LocalisedValue::new_optional(value.target_type, value.target_type_localised),
            target_faction: value.target_faction,
            destination_system: value.destination_system,
            destination_station: value.destination_station,
            destination_settlement: value.destination_settlement,
            reward: value.reward,
            donation: value.donation,
            donated: value.donated,
            permits_awarded: value.permits_awarded,
            materials_reward: value.materials_reward,
            commodity_reward: value.commodity_reward,
            faction_effects: value.faction_effects,
        }
    }
}

/// Due to using localised values we need an intermediary event
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct MissionCompletedEventSchema {
    
    /// Event meta data
    #[serde(flatten)]
    pub event_meta: EventMeta,
    
    /// The faction of the mission giver
    pub faction: String,
    
    /// The name of the mission
    pub name: String,
    
    /// The localised name of the mission
    pub localised_name: String,
    
    /// The id of the mission
    #[serde(rename = "MissionID")]
    pub mission_id: u64,
    
    /// The commodity involved 
    pub commodity: Option<String>,
    
    /// Localised commodity name
    #[serde(rename = "Commodity_Localised")]
    pub localised_commodity: Option<String>,
    
    /// The commodity count
    pub count: Option<u64>,
    
    /// The destination system
    pub destination_system: Option<String>,
    
    /// The destination station
    pub destination_station: Option<String>,
    
    /// The destination settlement
    pub destination_settlement: Option<String>,
    
    /// The target
    pub target: Option<String>,
    
    /// The target type
    pub target_type: Option<String>,
    
    /// The target type localised
    #[serde(rename = "TargetType_Localised")]
    pub target_type_localised: Option<String>,
    
    /// The target faction
    pub target_faction: Option<String>,
    
    /// The reward
    pub reward: Option<u64>,
    
    /// The donation
    pub donation: Option<String>,
    
    /// The donated amount
    pub donated: Option<u64>,
    
    /// Permits awarded
    pub permits_awarded: Option<Vec<String>>,
    
    /// Material rewards from the mission
    pub materials_reward: Option<Vec<MaterialReward>>,
    
    /// Comodity rewards from the mission
    pub commodity_reward: Option<Vec<CommodityReward>>,
    
    /// Faction effects
    pub faction_effects: Option<Vec<FactionEffectEntry>>,
    
}

impl From<MissionCompletedEvent> for MissionCompletedEventSchema {
    fn from(value: MissionCompletedEvent) -> Self {
        
        let (name, localised_name) = deconstruct_localised_value(value.name);
        let (commodity, localised_commodity) = deconstruct_optional_localised_value(value.commodity);
        let (target_type, target_type_localised) = deconstruct_optional_localised_value(value.target_type);
        Self {
            event_meta: value.event_meta,
            faction: value.faction,
            name,
            localised_name: localised_name.unwrap_or_default(),
            mission_id: value.mission_id,
            commodity,
            localised_commodity,
            count: value.count,
            target: value.target,
            target_type,
            target_type_localised,
            target_faction: value.target_faction,
            destination_system: value.destination_system,
            destination_station: value.destination_station,
            destination_settlement: value.destination_settlement,
            reward: value.reward,
            donation: value.donation,
            donated: value.donated,
            permits_awarded: value.permits_awarded,
            materials_reward: value.materials_reward,
            commodity_reward: value.commodity_reward,
            faction_effects: value.faction_effects,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::events::station_services::missions::common::MissionEffect;
    use crate::events::station_services::missions::mission_completed::components::{FactionEffect, FactionMissionInfluence, FactionTrend};

    #[test]
    fn test_deserialize_mission_completed_event() {
        let json_data = r#"{
  "timestamp": "2025-01-23T16:00:56Z",
  "event": "MissionCompleted",
  "Faction": "Ngalinn Jet Natural Incorporated",
  "Name": "Mission_Assassinate_RankEmp_name",
  "LocalisedName": "Imperial Navy Strike Contract Authorised",
  "MissionID": 1000034566,
  "TargetType": "$MissionUtil_FactionTag_PirateLord;",
  "TargetType_Localised": "Known Pirate",
  "TargetFaction": "Society of Ngalinn",
  "DestinationSystem": "Ngalinn",
  "DestinationStation": "Hickam Survey",
  "Target": "Mandrake",
  "Reward": 1166000,
  "MaterialsReward": [
    {
      "Name": "WakeSolutions",
      "Name_Localised": "Strange Wake Solutions",
      "Category": "$MICRORESOURCE_CATEGORY_Encoded;",
      "Category_Localised": "Encoded",
      "Count": 16
    }
  ],
  "FactionEffects": [
    {
      "Faction": "Ngalinn Jet Natural Incorporated",
      "Effects": [
        {
          "Effect": "$MISSIONUTIL_Interaction_Summary_EP_up;",
          "Effect_Localised": "The economic status of $#MinorFaction; has improved in the $#System; system.",
          "Trend": "UpGood"
        }
      ],
      "Influence": [
        {
          "SystemAddress": 3107509342922,
          "Trend": "UpGood",
          "Influence": "++"
        }
      ],
      "ReputationTrend": "UpGood",
      "Reputation": "++"
    },
    {
      "Faction": "",
      "Effects": [
        {
          "Effect": "$MISSIONUTIL_Interaction_Summary_EP_down;",
          "Effect_Localised": "The economic status of $#MinorFaction; has declined in the $#System; system.",
          "Trend": "DownBad"
        }
      ],
      "Influence": [
        {
          "SystemAddress": 2557753529034,
          "Trend": "DownBad",
          "Influence": "+"
        }
      ],
      "ReputationTrend": "DownBad",
      "Reputation": "+"
    }
  ]
}"#;


        let deserialized_event: MissionCompletedEvent = serde_json::from_str(&json_data).unwrap();

        // Basic assertions to verify the deserialization worked as expected
        assert_eq!(deserialized_event.faction, "Ngalinn Jet Natural Incorporated");
        assert_eq!(deserialized_event.name, LocalisedValue::new("Mission_Assassinate_RankEmp_name".to_string(), Some("Imperial Navy Strike Contract Authorised".to_string())));
        assert_eq!(deserialized_event.mission_id, 1000034566);
        assert_eq!(deserialized_event.target_type, LocalisedValue::new_optional(Some("$MissionUtil_FactionTag_PirateLord;".to_string()), Some("Known Pirate".to_string())));
        assert_eq!(deserialized_event.destination_system, Some("Ngalinn".to_string()));
        assert_eq!(deserialized_event.destination_station, Some("Hickam Survey".to_string()));
        assert_eq!(deserialized_event.target_faction, Some("Society of Ngalinn".to_string()));
        assert_eq!(deserialized_event.reward, Some(1166000));
        assert_eq!(deserialized_event.materials_reward, Some(vec![MaterialReward {
            name: LocalisedValue::new("WakeSolutions".to_string(), Some("Strange Wake Solutions".to_string())),
            category: LocalisedValue::new("$MICRORESOURCE_CATEGORY_Encoded;".to_string(), Some("Encoded".to_string())),
            count: 16,
        }]));
        
        
        
        let faction_effect_entry = FactionEffectEntry {
            faction: "Ngalinn Jet Natural Incorporated".to_string(),
            effects: vec![
                FactionEffect {
                    effect: LocalisedValue::new("$MISSIONUTIL_Interaction_Summary_EP_up;".to_string(), Some("The economic status of $#MinorFaction; has improved in the $#System; system.".to_string())),
                    trend: FactionTrend::UpGood,
                }
            ],
            influence: vec![
                FactionMissionInfluence {
                    system_address: 3107509342922,
                    trend: FactionTrend::UpGood,
                    influence: MissionEffect::Low,
                }
            ],
            reputation_trend: FactionTrend::UpGood,
            reputation: MissionEffect::Low,
        };
        
        let faction_effect_entry_2 = FactionEffectEntry {
            faction: "".to_string(),
            effects: vec![
                FactionEffect {
                    effect: LocalisedValue::new("$MISSIONUTIL_Interaction_Summary_EP_down;".to_string(), Some("The economic status of $#MinorFaction; has declined in the $#System; system.".to_string())),
                    trend: FactionTrend::DownBad,
                }
            ],
            influence: vec![
                FactionMissionInfluence {
                    system_address: 2557753529034,
                    trend: FactionTrend::DownBad,
                    influence: MissionEffect::VeryLow,}
            ],
            reputation_trend: FactionTrend::DownBad,
            reputation: MissionEffect::VeryLow,
        };
        
        assert_eq!(deserialized_event.faction_effects, Some(vec![faction_effect_entry, faction_effect_entry_2]));
            
    }
}