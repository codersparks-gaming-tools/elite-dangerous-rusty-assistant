use serde::{Deserialize, Serialize};
use crate::events::common::{EventMeta, LocalisedValue};
use crate::events::common::system::{Allegiance, Conflict, Faction, ThargoidWar};
use crate::events::travel::fsd_jump::star_position::StarPosition;


/// FSD Jump event
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(from = "FSDJumpEventSchema", into = "FSDJumpEventSchema")]
pub struct FSDJumpEvent {

    /// The event meta data
    pub event_meta: EventMeta,

    /// The star name
    pub body: String,

    /// The id of the star
    pub body_id: u64,

    /// The type of the star
    pub body_type: String,

    /// The factions in the system
    pub factions: Option<Vec<Faction>>,

    /// The name of the Star System
    pub star_system: String,

    /// The address of the system
    pub system_address: u64,

    /// The alegiance of the system
    pub system_allegiance: Allegiance,

    /// The position of the star
    pub star_pos: StarPosition,

    /// The faction of the system
    pub system_faction: Option<Faction>,

    /// The econompy of the system
    pub system_economy: LocalisedValue,

    /// System second economy
    pub system_second_economy: LocalisedValue,

    /// System government type
    pub system_government: LocalisedValue,

    /// System security
    pub system_security: LocalisedValue,

    /// Population of the system
    pub population: u64,

    /// The distance of the jump
    pub jump_dist: f32,

    /// Amount of fuel used
    pub fuel_used: f32,

    /// Fuel level
    pub fuel_level: f32,

    /// Taxi
    pub taxi: bool,

    /// Multi Crew
    #[serde(rename="Multicrew")]
    pub multi_crew: bool,

    /// Conflicts in the system
    pub conflicts: Option<Vec<Conflict>>,

    /// Thargoid war state
    pub thargoid_war: Option<ThargoidWar>,

}

impl From<FSDJumpEventSchema> for FSDJumpEvent {
    fn from(value: FSDJumpEventSchema) -> Self {
        let system_economy = LocalisedValue {
            value: value.system_economy,
            localised_value: Some(value.system_economy_localised),
        };

        let system_second_economy = LocalisedValue {
            value: value.system_second_economy,
            localised_value: Some(value.system_second_ecomomy_localised),
        };

        let system_government = LocalisedValue {
            value: value.system_government,
            localised_value: Some(value.system_government_localised),
        };

        let system_security = LocalisedValue {
            value: value.system_security,
            localised_value: Some(value.system_security_localised),
        };

        Self {
            event_meta: value.event_meta,
            body: value.body,
            body_id: value.body_id,
            body_type: value.body_type,
            factions: value.factions,
            system_faction: value.system_faction,
            star_system: value.star_system,
            system_address: value.system_address,
            system_allegiance: value.system_allegiance,
            star_pos: value.star_pos,
            system_economy,
            system_second_economy,
            system_government,
            system_security,
            population: value.population,
            jump_dist: value.jump_dist,
            fuel_used: value.fuel_used,
            fuel_level: value.fuel_level,
            taxi: value.taxi,
            multi_crew: value.multi_crew,
            conflicts: value.conflicts,
            thargoid_war: value.thargoid_war,
        }
    }
}


/// FSD Jump event schema
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all="PascalCase", deny_unknown_fields)]
pub struct FSDJumpEventSchema {

    /// The event meta data
    #[serde(flatten)]
    pub event_meta: EventMeta,

    /// The star name
    pub body: String,

    /// The id of the star
    #[serde(rename="BodyID")]
    pub body_id: u64,

    /// The type of the star
    pub body_type: String,

    /// The factions in the system
    pub factions: Option<Vec<Faction>>,

    /// The name of the Star System
    pub star_system: String,

    /// The address of the system
    pub system_address: u64,

    /// The alegiance of the system
    pub system_allegiance: Allegiance,

    /// The faction of the system
    pub system_faction: Option<Faction>,

    /// The position of the star
    pub star_pos: StarPosition,

    /// The econompy of the system
    pub system_economy: String,

    /// The localised version of the economy
    #[serde(rename="SystemEconomy_Localised")]
    pub system_economy_localised: String,

    /// System second economy
    pub system_second_economy: String,

    /// The localised version of the economy
    #[serde(rename="SystemSecondEconomy_Localised")]
    pub system_second_ecomomy_localised: String,

    /// System government type
    pub system_government: String,

    /// The localised version of the economy
    #[serde(rename="SystemGovernment_Localised")]
    pub system_government_localised: String,

    /// System security
    pub system_security: String,

    /// Localised version of system security
    #[serde(rename="SystemSecurity_Localised")]
    pub system_security_localised: String,

    /// Population of the system
    pub population: u64,

    /// The distance of the jump
    pub jump_dist: f32,

    /// Amount of fuel used
    pub fuel_used: f32,

    /// Fuel level
    pub fuel_level: f32,

    /// Taxi
    pub taxi: bool,

    /// Multi Crew
    #[serde(rename="Multicrew")]
    pub multi_crew: bool,

    /// Conflicts in the system
    pub conflicts: Option<Vec<Conflict>>,

    /// Thargoid war state
    pub thargoid_war: Option<ThargoidWar>,

}

impl From<FSDJumpEvent> for FSDJumpEventSchema {
    fn from(value: FSDJumpEvent) -> Self {
        Self {
            event_meta: value.event_meta,
            body: value.body,
            body_id: value.body_id,
            body_type: value.body_type,
            factions: value.factions,
            system_faction: value.system_faction,
            star_system: value.star_system,
            system_address: value.system_address,
            system_allegiance: value.system_allegiance,
            star_pos: value.star_pos,
            system_economy: value.system_economy.value,
            system_economy_localised: value.system_economy.localised_value.unwrap_or("".to_string()),
            system_second_economy: value.system_second_economy.value,
            system_second_ecomomy_localised: value.system_second_economy.localised_value.unwrap_or("".to_string()),
            system_government: value.system_government.value,
            system_government_localised: value.system_government.localised_value.unwrap_or("".to_string()),
            system_security: value.system_security.value,
            system_security_localised: value.system_security.localised_value.unwrap_or("".to_string()),
            population: value.population,
            jump_dist: value.jump_dist,
            fuel_used: value.fuel_used,
            fuel_level: value.fuel_level,
            taxi: value.taxi,
            multi_crew: value.multi_crew,
            conflicts: value.conflicts,
            thargoid_war: value.thargoid_war,
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;
    use crate::events::common::system::{ConflictFaction, FactionState};
    use crate::test_helper::serde_helpers::create_timestamp;

    #[test]
    fn test_deserialize_fsd_jump_event() {

        let timestamp_str = "2025-01-04T19:43:55Z";
        let timestamp = create_timestamp(timestamp_str);
        let json = format!(r#"{{
  "timestamp": "2025-01-04T19:43:55Z",
  "Taxi": false,
  "Multicrew": false,
  "StarSystem": "Tiguai",
  "SystemAddress": 11664191792537,
  "StarPos": [
    -132.28125,
    31.40625,
    -39.62500
  ],
  "SystemAllegiance": "Independent",
  "SystemEconomy": "$economy_Colony;",
  "SystemEconomy_Localised": "Colony",
  "SystemSecondEconomy": "$economy_Extraction;",
  "SystemSecondEconomy_Localised": "Extraction",
  "SystemGovernment": "$government_Dictatorship;",
  "SystemGovernment_Localised": "Dictatorship",
  "SystemSecurity": "$SYSTEM_SECURITY_low;",
  "SystemSecurity_Localised": "Low Security",
  "Population": 1662,
  "Body": "Tiguai",
  "BodyID": 0,
  "BodyType": "Star",
  "JumpDist": 44.465,
  "FuelUsed": 8.054239,
  "FuelLevel": 21.805761,
  "Factions": [
    {{
      "Name": "Nicobarese Resistance",
      "FactionState": "None",
      "Government": "Democracy",
      "Influence": 0.201798,
      "Allegiance": "Federation",
      "Happiness": "$Faction_HappinessBand2;",
      "Happiness_Localised": "Happy",
      "MyReputation": 0.000000
    }},
    {{
      "Name": "Independent Wanggu Alliance",
      "FactionState": "PublicHoliday",
      "Government": "Confederacy",
      "Influence": 0.072927,
      "Allegiance": "Federation",
      "Happiness": "$Faction_HappinessBand1;",
      "Happiness_Localised": "Elated",
      "MyReputation": 0.000000,
      "ActiveStates": [
        {{
          "State": "PublicHoliday"
        }}
      ]
    }},
    {{
      "Name": "Wings Of Justice",
      "FactionState": "None",
      "Government": "Dictatorship",
      "Influence": 0.356643,
      "Allegiance": "Independent",
      "Happiness": "$Faction_HappinessBand2;",
      "Happiness_Localised": "Happy",
      "MyReputation": 0.000000,
      "RecoveringStates": [
        {{
          "State": "Expansion",
          "Trend": 0
        }}
      ]
    }}
  ],
  "SystemFaction": {{
    "Name": "Wings Of Justice"
  }},
  "Conflicts": [
    {{
      "WarType": "civilwar",
      "Status": "active",
      "Faction1": {{
        "Name": "Arque Commodities",
        "Stake": "Uchida Drilling Site",
        "WonDays": 0
      }},
      "Faction2": {{
        "Name": "Uniting Arque",
        "Stake": "",
        "WonDays": 1
      }}
    }}
  ],
   "ThargoidWar": {{
    "CurrentState": "",
    "NextStateSuccess": "",
    "NextStateFailure": "",
    "SuccessStateReached": true,
    "WarProgress": 1.000000,
    "RemainingPorts": 0
  }}
}}
"#);

        let faction1 = Faction {
            name: "Nicobarese Resistance".to_string(),
            faction_state: Some("None".to_string()),
            government: Some("Democracy".to_string()),
            influence: Some(0.201798),
            allegiance: Some(Allegiance::Federation),
            happiness: Some(LocalisedValue {
                value: "$Faction_HappinessBand2;".to_string(),
                localised_value: Some("Happy".to_string())
            }),
            my_reputation: Some(0.000000),
            active_states: None,
            pending_states: None,
            recovering_states: None,
        };

        let faction2 = Faction {
            name: "Independent Wanggu Alliance".to_string(),
            faction_state: Some("PublicHoliday".to_string()),
            government: Some("Confederacy".to_string()),
            influence: Some(0.072927),
            allegiance: Some(Allegiance::Federation),
            happiness: Some(LocalisedValue {
                value: "$Faction_HappinessBand1;".to_string(),
                localised_value: Some("Elated".to_string())
            }),
            my_reputation: Some(0.000000),
            active_states: Some(vec![
                FactionState {
                    state: "PublicHoliday".to_string(),
                    trend: None,
                },
            ]),
            pending_states: None,
            recovering_states: None,
        };

        let faction3  = Faction {
            name: "Wings Of Justice".to_string(),
            faction_state: Some("None".to_string()),
            government: Some("Dictatorship".to_string()),
            influence: Some(0.356643),
            allegiance: Some(Allegiance::Independent),
            happiness: Some(LocalisedValue {
                value: "$Faction_HappinessBand2;".to_string(),
                localised_value: Some("Happy".to_string())
            }),
            my_reputation: Some(0.000000),
            active_states: None,
            pending_states: None,
            recovering_states: Some(vec![
            FactionState {
                state: "Expansion".to_string(),
                trend: Some(0)
            }
            ])
        };

        let conflict_faction_1 = ConflictFaction {
            name: "Arque Commodities".to_string(),
            stake: "Uchida Drilling Site".to_string(),
            won_days: 0
        };

        let conflict_faction_2 = ConflictFaction {
            name: "Uniting Arque".to_string(),
            stake: "".to_string(),
            won_days: 1
        };

        let conflict = Conflict {
            war_type: "civilwar".to_string(),
            status: "active".to_string(),
            faction1: conflict_faction_1,
            faction2: conflict_faction_2,

        };

        let thargoid_war = ThargoidWar {
            current_state: "".to_string(),
            next_state_success: "".to_string(),
            next_state_failure: "".to_string(),
            success_state_reached: true,
            war_progress: 1.000000,
            remaining_ports: 0,
        };


        let event : FSDJumpEvent = serde_json::from_str(&json).unwrap();

        println!("{:#?}", event);

        assert_eq!(event.event_meta.timestamp, timestamp);
        assert_eq!(event.star_system, "Tiguai");
        assert_eq!(event.system_address, 11664191792537);
        assert_eq!(event.star_pos.x, -132.28125);
        assert_eq!(event.star_pos.y, 31.40625);
        assert_eq!(event.star_pos.z, -39.625);
        assert_eq!(event.system_economy.value, "$economy_Colony;");
        assert_eq!(event.system_economy.localised_value, Some("Colony".to_string()));
        assert_eq!(event.system_second_economy.value, "$economy_Extraction;");
        assert_eq!(event.system_second_economy.localised_value, Some("Extraction".to_string()));
        assert_eq!(event.system_government.value, "$government_Dictatorship;");
        assert_eq!(event.system_government.localised_value, Some("Dictatorship".to_string()));
        assert_eq!(event.system_security.value, "$SYSTEM_SECURITY_low;");
        assert_eq!(event.system_security.localised_value, Some("Low Security".to_string()));
        assert_eq!(event.population, 1662);
        assert_eq!(event.jump_dist, 44.465);
        assert_eq!(event.fuel_used, 8.054239);
        assert_eq!(event.fuel_level, 21.805761);
        assert_eq!(event.taxi, false);
        assert_eq!(event.multi_crew, false);
        assert_eq!(event.factions, Some(vec![faction1, faction2, faction3]));
        assert_eq!(event.conflicts, Some(vec![conflict]));
        assert_eq!(event.thargoid_war, Some(thargoid_war));

    }
}
