use crate::events::common::LocalisedValue;
use serde::{Deserialize, Serialize};

/// What is the allegiance of the system/faction etc
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub enum Allegiance {
    /// Allegiance is with: Alliance
    Alliance,
    /// Allegiance is with: Empire
    Empire,
    /// Allegiance is with: Federation
    Federation,
    /// System is independant
    Independent,
    /// No allegiance
    #[serde(untagged)]
    Other(String)
}

/// Struct to represent state of a faction
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct FactionState {
    /// The name of the state
    pub state: String,
    /// The trend if any
    pub trend: Option<i32>,
}

/// Struct used in conflict struct to give details about the factions at war
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct ConflictFaction {
    /// Name of the faction
    pub name: String,
    /// The stake of the war
    pub stake: String,
    /// Number of days won?
    pub won_days: u32,
}

/// Struct to represent conflict
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct Conflict {
    /// The type of war
    pub war_type: String,
    /// Status of the war
    pub status: String,
    /// Faction 1 at war
    pub faction1: ConflictFaction,
    /// Faction 2 at war
    pub faction2: ConflictFaction,
}

/// Information about thargoid war
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct ThargoidWar {
    /// The current state
    pub current_state: String,
    /// The next state if successful
    pub next_state_success: String,
    /// The next state if unsuccessful
    pub next_state_failure: String,
    /// Has success state been reached
    pub success_state_reached: bool,
    /// The war progress
    pub war_progress: f32,
    /// How many ports are remaining
    pub remaining_ports: u32,
}

/// Struct to represent faction
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(from = "FactionSchema", into = "FactionSchema")]
pub struct Faction {
    /// The name of the faction
    pub name: String,
    /// The state of the faction
    pub faction_state: Option<String>,
    /// The government type
    pub government: Option<String>,
    /// The influence of the faction,
    pub influence: Option<f32>,
    /// The allegiance of the faction
    pub allegiance: Option<Allegiance>,
    /// The happiness of the faction
    pub happiness: Option<LocalisedValue>,
    /// Pilots reputation with faction
    pub my_reputation: Option<f32>,
    /// The currently active states
    pub active_states: Option<Vec<FactionState>>,
    /// The pending states if any
    pub pending_states: Option<Vec<FactionState>>,
    /// The recoving states if any
    pub recovering_states: Option<Vec<FactionState>>,
}

impl From<FactionSchema> for Faction {
    fn from(value: FactionSchema) -> Self {
        let happiness = Some(LocalisedValue {
            value: value.happiness.unwrap_or_default(),
            localised_value: Some(value.happiness_localised.unwrap_or_default()),
        });

        Self {
            name: value.name,
            faction_state: value.faction_state,
            government: value.government,
            influence: value.influence,
            allegiance: value.allegiance,
            happiness,
            my_reputation: value.my_reputation,
            active_states: value.active_states,
            pending_states: value.pending_states,
            recovering_states: value.recovering_states,
        }
    }
}

/// Schema event to convert from/to due to localised value
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub(crate) struct FactionSchema {
    /// The name of the faction
    pub name: String,
    /// The state of the faction
    pub faction_state: Option<String>,
    /// The government type
    pub government: Option<String>,
    /// The influence of the faction,
    pub influence: Option<f32>,
    /// The allegiance of the faction
    pub allegiance: Option<Allegiance>,
    pub happiness: Option<String>,
    /// The localised happiness of the faction
    #[serde(rename = "Happiness_Localised")]
    pub happiness_localised: Option<String>,
    /// Pilots reputation with faction
    pub my_reputation: Option<f32>,
    /// The currently active states
    pub active_states: Option<Vec<FactionState>>,
    /// The pending states if any
    pub pending_states: Option<Vec<FactionState>>,
    /// The recoving states if any
    pub recovering_states: Option<Vec<FactionState>>,
}

impl From<Faction> for FactionSchema {
    fn from(value: Faction) -> Self {
        let happiness;
        let happiness_localised;
        match value.happiness {
            None => {
                happiness = None;
                happiness_localised = None;
            }
            Some(v) => {
                happiness = Some(v.value);
                happiness_localised = match v.localised_value {
                    None => None,
                    Some(lv) => Some(lv),
                };
            }
        }
        Self {
            name: value.name,
            faction_state: value.faction_state,
            government: value.government,
            influence: value.influence,
            allegiance: value.allegiance,
            happiness,
            happiness_localised,
            my_reputation: value.my_reputation,
            active_states: value.active_states,
            pending_states: value.pending_states,
            recovering_states: value.recovering_states,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::events::common::system::{Allegiance, Faction, FactionState};
    use crate::events::common::LocalisedValue;

    #[test]
    fn test_deserialize_faction() {
        let json = format!(
            r#"{{
      "Name": "United Nemepawe Progressive Party",
      "FactionState": "None",
      "Government": "Democracy",
      "Influence": 0.137862,
      "Allegiance": "Federation",
      "Happiness": "$Faction_HappinessBand2;",
      "Happiness_Localised": "Happy",
      "ActiveStates": [
        {{
          "State": "PublicHoliday"
        }}
      ],
      "MyReputation": 0.000000,
      "RecoveringStates": [
        {{
          "State": "Expansion",
          "Trend": 0
        }}
      ],
    }}"#
        );

        let faction: Faction = serde_json::from_str(&json).unwrap();

        println!("{:#?}", faction);

        assert_eq!(faction.name, "United Nemepawe Progressive Party");
        assert_eq!(faction.faction_state, Some("None".to_string()));
        assert_eq!(faction.government, Some("Democracy".to_string()));
        assert_eq!(faction.influence, Some(0.137862));
        assert_eq!(faction.allegiance, Some(Allegiance::Federation));
        assert_eq!(faction.happiness, Some(LocalisedValue { value: "$Faction_HappinessBand2;".to_string(), localised_value: Some("Happy".to_string()) }));
        assert_eq!(faction.my_reputation, Some(0.0));
        assert_eq!(faction.active_states, Some(vec![FactionState { state: "PublicHoliday".to_string(), trend: None }]));
        assert_eq!(faction.pending_states, None);
        assert_eq!(faction.recovering_states, Some(vec![FactionState { state: "Expansion".to_string(), trend: Some(0) }]));
    }
}
