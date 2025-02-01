use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use crate::events::common::{deconstruct_optional_localised_value, EventMeta, LocalisedValue};

/// The effect of the successful mission
#[allow(missing_docs)]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub enum MissionEffect {
    None,
    Low,
    Med,
    High,
}

impl From<String> for MissionEffect {
    fn from(value: String) -> Self {
        match value.as_str() {
            "None" => MissionEffect::None,
            "+" => MissionEffect::Low,
            "++" => MissionEffect::Med,
            "+++" => MissionEffect::High,
            _ => panic!("Unknown mission effect"),
        }
    }
}


impl MissionEffect {

    /// Convert the mission effect to the string representation that is present in events
    pub fn to_string(&self) -> String {
        match self {
            MissionEffect::None => String::from("None"),
            MissionEffect::Low => String::from("+"),
            MissionEffect::Med => String::from("++"),
            MissionEffect::High => String::from("+++"),
        }
    }
}

/// Enum for passenger type
#[allow(missing_docs)]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub enum PassengerType {
    Tourist,
    Soldier,
    Explorer,
    /// Option to catch unspecified passenger type
    Other(String),
}



/// The struct representing the Mission Accepted event
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(from = "MissionAcceptedEventSchema", into = "MissionAcceptedEventSchema")]
pub struct MissionAcceptedEvent {
    /// The event meta data
    pub event_meta: EventMeta,

    /// The issuing faction
    pub faction: String,

    /// The name of the mission
    pub name: LocalisedValue,

    /// The mission ID
    pub mission_id: u64,

    /// The effect on faction influence
    pub influence: MissionEffect,

    /// The effect on faction reputation
    pub reputation: MissionEffect,

    /// The commodity that is to be delivered or required
    pub commodity: Option<LocalisedValue>,

    /// The number of commodity that needs to be delivered or is required
    pub count: Option<u64>,

    /// The contracted donation amount
    pub donation: Option<String>,

    /// The donated amount
    pub donated: Option<u64>,

    /// The name of the target
    pub target: Option<String>,

    /// The type of target
    pub target_type: Option<LocalisedValue>,

    /// The target faction
    pub target_faction: Option<String>,

    /// The number of targets
    pub kill_count: Option<u64>,

    /// The mission expiry time in ISO 8601
    pub expiry: Option<NaiveDateTime>,

    /// The destination system
    pub destination_system: Option<String>,

    /// The destination station
    pub destination_station: Option<String>,

    /// The destination settlement
    pub destination_settlement: Option<String>,

    /// New destination system if redirected
    pub new_destination_system: Option<String>,

    /// New destination station if redirected
    pub new_destination_station: Option<String>,

    /// Number of passengers
    pub passenger_count: Option<u64>,

    /// Are the passengers VIPs?
    pub passenger_vips: Option<bool>,

    /// Are the passengers wanted?
    pub passenger_wanted: Option<bool>,

    /// The type of passenger
    pub passenger_type: Option<PassengerType>,
}

impl From<MissionAcceptedEventSchema> for MissionAcceptedEvent {
    fn from(value: MissionAcceptedEventSchema) -> Self {        
        let expiry = match value.expiry {
            None => None,
            Some(v) => Some(NaiveDateTime::parse_from_str(v.as_str(), "%Y-%m-%dT%H:%M:%SZ").unwrap()),
        };
        
        let commodity = LocalisedValue::new_optional(value.commodity, value.commodity_localised);
        let target_type = LocalisedValue::new_optional(value.target_type, value.target_type_localised);
        Self {
            event_meta: EventMeta { timestamp: value.event_meta.timestamp },
            faction: value.faction,
            name: LocalisedValue { value: value.name, localised_value: Some(value.localised_name)},
            mission_id: value.mission_id,
            influence: value.influence.into(),
            reputation: value.reputation.into(),
            commodity,
            count: value.count,
            donation: value.donation,
            donated: value.donated,
            target: value.target,
            target_type,
            target_faction: value.target_faction,
            kill_count: value.kill_count,
            expiry: expiry,
            destination_system: value.destination_system,
            destination_station: value.destination_station,
            destination_settlement: value.destination_settlement,
            new_destination_system: value.new_destination_system,
            new_destination_station: value.new_destination_station,
            passenger_count: value.passenger_count,
            passenger_vips: value.passenger_vips,
            passenger_wanted: value.passenger_wanted,
            passenger_type: value.passenger_type,
        }
    }
}

/// Schema struct for Mission Accepted Event due to localised value
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct MissionAcceptedEventSchema {

    /// The event meta data
    #[serde(flatten)]
    pub event_meta: EventMeta,

    /// The issuing faction
    pub faction: String,

    /// The name of the mission
    pub name: String,

    /// The localised name of the mission
    pub localised_name: String,

    /// The mission ID
    #[serde(rename = "MissionID")]
    pub mission_id: u64,

    /// The effect on faction influence
    pub influence: String,

    /// The effect on faction reputation
    pub reputation: String,

    /// The commodity that is to be delivered or required
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commodity: Option<String>,

    /// The localised version of the commodity
    #[serde(rename = "Commodity_Localised", skip_serializing_if = "Option::is_none")]
    pub commodity_localised: Option<String>,

    /// The number of commodity that needs to be delivered or is required
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<u64>,

    /// The contracted donation amount
    #[serde(skip_serializing_if = "Option::is_none")]
    pub donation: Option<String>,

    /// The donated amount
    #[serde(skip_serializing_if = "Option::is_none")]
    pub donated: Option<u64>,

    /// The name of the target
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,

    /// The type of target
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_type: Option<String>,
    
    /// Localised version of the target type
    #[serde(rename = "TargetType_Localised", skip_serializing_if = "Option::is_none")]
    pub target_type_localised: Option<String>,

    /// The target faction
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_faction: Option<String>,

    /// The number of targets
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kill_count: Option<u64>,

    /// The mission expiry time in ISO 8601
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiry: Option<String>,

    /// The destination system
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_system: Option<String>,

    /// The destination station
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_station: Option<String>,

    /// The destination settlement
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_settlement: Option<String>,

    /// New destination system if redirected
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_destination_system: Option<String>,

    /// New destination station if redirected
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_destination_station: Option<String>,

    /// Number of passengers
    #[serde(skip_serializing_if = "Option::is_none")]
    pub passenger_count: Option<u64>,

    /// Are the passengers VIPs?
    #[serde(rename = "PassengerVIPs", skip_serializing_if = "Option::is_none")]
    pub passenger_vips: Option<bool>,

    /// Are the passengers wanted?
    #[serde(skip_serializing_if = "Option::is_none")]
    pub passenger_wanted: Option<bool>,

    /// The type of passenger
    #[serde(skip_serializing_if = "Option::is_none")]
    pub passenger_type: Option<PassengerType>,

}

impl From<MissionAcceptedEvent> for MissionAcceptedEventSchema {
    fn from(value: MissionAcceptedEvent) -> Self {
        let expiry = match value.expiry {
            None => None,
            Some(v) => Some(v.format("%Y-%m-%dT%H:%M:%SZ").to_string()),
        };

        let (commodity, commodity_localised) = deconstruct_optional_localised_value(value.commodity);
        let (target_type, target_type_localised) = deconstruct_optional_localised_value(value.target_type);

        Self {
            event_meta: value.event_meta,
            faction: value.faction,
            name: value.name.value,
            localised_name: value.name.localised_value.unwrap_or_default(),
            mission_id: value.mission_id,
            influence: value.influence.to_string(),
            reputation: value.reputation.to_string(),
            commodity,
            commodity_localised,
            count: value.count,
            donation: value.donation,
            donated: value.donated,
            target: value.target,
            target_type,
            target_type_localised,
            target_faction: value.target_faction,
            kill_count: value.kill_count,
            expiry,
            destination_system: value.destination_system,
            destination_station: value.destination_station,
            destination_settlement: value.destination_settlement,
            new_destination_system: value.new_destination_system,
            new_destination_station: value.new_destination_station,
            passenger_count: value.passenger_count,
            passenger_vips: value.passenger_vips,
            passenger_wanted: value.passenger_wanted,
            passenger_type: value.passenger_type,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::{json, Value};
    use crate::test_helper::serde_helpers::create_timestamp;

    #[test]
    fn test_serialization_with_all_fields() {
        // Arrange
        let mission_accepted_event = MissionAcceptedEvent {
            event_meta: EventMeta { timestamp: create_timestamp("2023-10-16T12:00:00Z") },
            faction: "FactionName".to_string(),
            name: LocalisedValue {
                value: "MissionName".to_string(),
                localised_value: Some("LocalizedMissionName".to_string()),
            },
            mission_id: 123456,
            influence: MissionEffect::Med,
            reputation: MissionEffect::High,
            commodity: Some(LocalisedValue {
                value: "CommodityName".to_string(),
                localised_value: Some("LocalizedCommodityName".to_string()),
            }),
            count: Some(10),
            donation: Some("50000".to_string()),
            donated: Some(10000),
            target: Some("TargetEntity".to_string()),
            target_type: LocalisedValue::new_optional(Some("EntityType".to_string()), Some("LocalizedEntityType".to_string())),
            target_faction: Some("TargetFaction".to_string()),
            kill_count: Some(5),
            expiry: Some(NaiveDateTime::parse_from_str("2023-10-25T14:00:00Z", "%Y-%m-%dT%H:%M:%SZ").unwrap()),
            destination_system: Some("DestinationSystem".to_string()),
            destination_station: Some("DestinationStation".to_string()),
            destination_settlement: Some("DestinationSettlement".to_string()),
            new_destination_system: Some("NewDestinationSystem".to_string()),
            new_destination_station: Some("NewDestinationStation".to_string()),
            passenger_count: Some(20),
            passenger_vips: Some(true),
            passenger_wanted: Some(false),
            passenger_type: Some(PassengerType::Tourist),
        };

        // Act
        let serialized = serde_json::to_string(&MissionAcceptedEventSchema::from(mission_accepted_event)).unwrap();
        let serialized_json: Value = serde_json::from_str(&serialized).unwrap();
        
        // Assert
        assert_eq!(serialized_json["Faction"], "FactionName");
        assert_eq!(serialized_json["MissionID"], 123456);
        assert_eq!(serialized_json["Influence"], "++");
        assert_eq!(serialized_json["Reputation"], "+++");
        assert_eq!(serialized_json["PassengerVIPs"], true);
        assert_eq!(serialized_json["PassengerType"], "Tourist");
        assert_eq!(serialized_json["Target"], "TargetEntity");
        assert_eq!(serialized_json["TargetType"], "EntityType");
        assert_eq!(serialized_json["TargetType_Localised"], "LocalizedEntityType");
        assert_eq!(serialized_json["TargetFaction"], "TargetFaction");
        assert_eq!(serialized_json["KillCount"], 5);
        assert_eq!(serialized_json["Expiry"], "2023-10-25T14:00:00Z");
        assert_eq!(serialized_json["DestinationSystem"], "DestinationSystem");
        assert_eq!(serialized_json["DestinationStation"], "DestinationStation");
        assert_eq!(serialized_json["DestinationSettlement"], "DestinationSettlement");
        assert_eq!(serialized_json["NewDestinationSystem"], "NewDestinationSystem");
        assert_eq!(serialized_json["NewDestinationStation"], "NewDestinationStation");
        assert_eq!(serialized_json["Commodity"], "CommodityName");
        assert_eq!(serialized_json["Commodity_Localised"], "LocalizedCommodityName");
        assert_eq!(serialized_json["Count"], 10);
        assert_eq!(serialized_json["Donation"], "50000");
        assert_eq!(serialized_json["Donated"], 10000);
        assert_eq!(serialized_json["Name"], "MissionName");
        assert_eq!(serialized_json["LocalisedName"], "LocalizedMissionName");
        assert_eq!(serialized_json["PassengerCount"], 20);
        assert_eq!(serialized_json["PassengerWanted"], false);
        assert_eq!(serialized_json["Target"], "TargetEntity");
    }

    #[test]
    fn test_serialization_with_no_optional_fields() {
        // Arrange
        let mission_accepted_event = MissionAcceptedEvent {
            event_meta: EventMeta { timestamp: create_timestamp("2023-10-16T12:00:00Z") },
            faction: "FactionName".to_string(),
            name: LocalisedValue {
                value: "MissionName".to_string(),
                localised_value: Some("LocalizedMissionName".to_string()),
            },
            mission_id: 123456,
            influence: MissionEffect::Low,
            reputation: MissionEffect::None,
            commodity: None,
            count: None,
            donation: None,
            donated: None,
            target: None,
            target_type: None,
            target_faction: None,
            kill_count: None,
            expiry: None,
            destination_system: None,
            destination_station: None,
            destination_settlement: None,
            new_destination_system: None,
            new_destination_station: None,
            passenger_count: None,
            passenger_vips: None,
            passenger_wanted: None,
            passenger_type: None,
        };
        
        // Act
        let serialized = serde_json::to_string(&MissionAcceptedEventSchema::from(mission_accepted_event)).unwrap();
        let serialized_json: Value = serde_json::from_str(&serialized).unwrap();

        // Assert
        assert_eq!(serialized_json["Faction"], "FactionName");
        assert_eq!(serialized_json["MissionID"], 123456);
        assert_eq!(serialized_json["Influence"], "+");
        assert_eq!(serialized_json["Reputation"], "None");
        assert_eq!(serialized_json["Faction"], "FactionName");
        
    }

    #[test]
    fn test_deserialization_with_all_fields() {
        // Arrange
        let input_json = json!({
            "timestamp": "2023-10-16T12:00:00Z",
            "Faction": "FactionName",
            "Name": "MissionName",
            "LocalisedName": "LocalizedMissionName",
            "MissionID": 123456,
            "Influence": "++",
            "Reputation": "+++",
            "Commodity": "CommodityName",
            "Commodity_Localised": "LocalizedCommodityName",
            "Count": 10,
            "Donation": "50000",
            "Donated": 10000,
            "Target": "TargetEntity",
            "Target_Type": "EntityType",
            "TargetFaction": "TargetFaction",
            "KillCount": 5,
            "Expiry": "2023-10-25T14:00:00Z",
            "DestinationSystem": "DestinationSystem",
            "DestinationStation": "DestinationStation",
            "DestinationSettlement": "DestinationSettlement",
            "NewDestinationSystem": "NewDestinationSystem",
            "NewDestinationStation": "NewDestinationStation",
            "PassengerCount": 20,
            "PassengerVIPs": true,
            "PassengerWanted": false,
            "PassengerType": "Tourist"
        })
            .to_string();

        // Act
        let deserialized: MissionAcceptedEvent =
            serde_json::from_str(&input_json).unwrap();

        // Assert
        assert_eq!(deserialized.faction, "FactionName");
        assert_eq!(deserialized.name.value, "MissionName");
        assert_eq!(deserialized.mission_id, 123456);
        assert_eq!(deserialized.influence, MissionEffect::Med);
        assert_eq!(deserialized.reputation, MissionEffect::High);
        assert_eq!(deserialized.passenger_count, Some(20));
        assert_eq!(deserialized.passenger_type, Some(PassengerType::Tourist));
        assert_eq!(deserialized.destination_station, Some("DestinationStation".to_string()));
    }

    #[test]
    fn test_deserialization_with_no_optional_fields() {
        // Arrange
        let input_json = json!({
            "timestamp": "2023-10-16T12:00:00Z",
            "Faction": "FactionName",
            "Name": "MissionName",
            "LocalisedName": "LocalizedMissionName",
            "MissionID": 123456,
            "Influence": "+",
            "Reputation": "None"
        })
            .to_string();

        // Act
        let deserialized: MissionAcceptedEvent =
            serde_json::from_str(&input_json).unwrap();

        // Assert
        assert_eq!(deserialized.faction, "FactionName");
        assert_eq!(deserialized.name.value, "MissionName");
        assert_eq!(deserialized.mission_id, 123456);
        assert_eq!(deserialized.influence, MissionEffect::Low);
        assert_eq!(deserialized.reputation, MissionEffect::None);
        assert!(deserialized.commodity.is_none());
    }
    
    #[test]
    pub fn test_actual_event_deserialization() {
        
        let timestamp_str = "2025-01-15T19:15:18Z";
        let timestamp = create_timestamp(timestamp_str);
        let json = format!(r#"{{ "timestamp":"{timestamp_str}", "event":"MissionAccepted", "Faction":"Flat Galaxy Society", "Name":"Mission_Massacre", "LocalisedName":"Kill Manten Family faction Pirates", "TargetType":"$MissionUtil_FactionTag_Pirate;", "TargetType_Localised":"Pirates", "TargetFaction":"Manten Family", "KillCount":9, "DestinationSystem":"Manten", "DestinationStation":"Leonov Settlement", "Expiry":"2025-01-17T10:20:15Z", "Wing":false, "Influence":"++", "Reputation":"++", "Reward":4284213, "MissionID":998632967 }}"#);
        
        let event = serde_json::from_str::<MissionAcceptedEvent>(&json).unwrap();
        
        let (target_type, target_type_localised) = deconstruct_optional_localised_value(event.target_type);
        
        
        assert_eq!(event.event_meta.timestamp, timestamp);
        assert_eq!(event.faction, "Flat Galaxy Society");
        assert_eq!(event.name.value, "Mission_Massacre");
        assert_eq!(event.name.localised_value, Some("Kill Manten Family faction Pirates".to_string()));
        assert_eq!(target_type, Some("$MissionUtil_FactionTag_Pirate;".to_string()));
        assert_eq!(target_type_localised, Some("Pirates".to_string()));
        assert_eq!(event.target_faction, Some("Manten Family".to_string()));
        assert_eq!(event.kill_count, Some(9));
        assert_eq!(event.destination_system, Some("Manten".to_string()));
        assert_eq!(event.destination_station, Some("Leonov Settlement".to_string()));
        assert_eq!(event.expiry, Some(NaiveDateTime::parse_from_str("2025-01-17T10:20:15Z", "%Y-%m-%dT%H:%M:%SZ").unwrap()));
    }
}