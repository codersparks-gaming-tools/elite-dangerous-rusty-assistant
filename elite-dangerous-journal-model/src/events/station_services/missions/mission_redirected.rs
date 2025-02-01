use serde::{Deserialize, Serialize};
use crate::events::common::{EventMeta, LocalisedValue};

/// The mission redirected event
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(from = "MissionRedirectedEventSchema", into = "MissionRedirectedEventSchema")]
pub struct MissionRedirectedEvent {
    
    /// The event meta data
    pub event_meta: EventMeta,
    
    /// The mission id
    pub mission_id: u64,
    
    /// The name of the mission
    pub name: LocalisedValue,
    
    /// The new destination system
    pub new_destination_system: String,
    
    /// The new destination station
    pub new_destination_station: String,
    
    /// The old destination system
    pub old_destination_system: String,
    
    /// The old destination station
    pub old_destination_station: String,
}

impl From<MissionRedirectedEventSchema> for MissionRedirectedEvent {
    
    /// Convert from Mission Redirected Event Schema
    fn from(schema: MissionRedirectedEventSchema) -> Self {
        Self {
            event_meta: schema.event_meta,
            mission_id: schema.mission_id,
            name: LocalisedValue::new(schema.name, Some(schema.localised_name)),
            new_destination_system: schema.new_destination_system,
            new_destination_station: schema.new_destination_station,
            old_destination_system: schema.old_destination_system,
            old_destination_station: schema.old_destination_station,
        }
    }
}

/// As there is a localised value we need to create an intermediary struct
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct MissionRedirectedEventSchema {
    
    /// The event meta data
    #[serde(flatten)]
    pub event_meta: EventMeta,

    /// The mission id
    #[serde(rename = "MissionID")]
    pub mission_id: u64,

    /// The name of the mission
    pub name: String,
    
    /// The localised name of the mission
    pub localised_name: String,

    /// The new destination system
    pub new_destination_system: String,

    /// The new destination station
    pub new_destination_station: String,

    /// The old destination system
    pub old_destination_system: String,

    /// The old destination station
    pub old_destination_station: String,
}

impl From<MissionRedirectedEvent> for MissionRedirectedEventSchema {
    
    /// Convert from MissionRedirectEvent
    fn from(value: MissionRedirectedEvent) -> Self {
        
        
        Self {
            event_meta: value.event_meta,
            mission_id: value.mission_id,
            name: value.name.value,
            localised_name: value.name.localised_value.unwrap(),
            new_destination_system: value.new_destination_system,
            new_destination_station: value.new_destination_station,
            old_destination_system: value.old_destination_system,
            old_destination_station: value.old_destination_station,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_helper::serde_helpers::create_timestamp;
    
    #[test]
    fn test_deserialize_mission_redirected_event() {
        let timestamp_str = "2025-01-13T18:05:28Z";
        let timestamp = create_timestamp(timestamp_str);
        let json = r#"{"timestamp":"2025-01-13T18:05:28Z","event":"MissionRedirected","Name":"Mission_Name","LocalisedName":"Localised Mission Name","MissionID":123456789,"NewDestinationSystem":"New Destination System","NewDestinationStation":"New Destination Station","OldDestinationSystem":"Old Destination System","OldDestinationStation":"Old Destination Station"}"#;
        let event: MissionRedirectedEvent = serde_json::from_str(&json).expect("Failed to deserialize MissionRedirectedEvent");
        assert_eq!(event.event_meta.timestamp, timestamp);
        assert_eq!(event.name.value, "Mission_Name");
        assert_eq!(event.name.localised_value, Some("Localised Mission Name".to_string()));
        assert_eq!(event.mission_id, 123456789);
        assert_eq!(event.new_destination_system, "New Destination System");
    }
    
    #[test]
    fn test_serialize_mission_redirected_event() {
        
        let event = MissionRedirectedEvent {
            event_meta: EventMeta {
                timestamp: create_timestamp("2025-01-13T18:05:28Z")
            },
            name: LocalisedValue::new("Mission_Name".to_string(), Some("Localised Mission Name".to_string())),
            mission_id: 123456789,
            new_destination_system: "New Destination System".to_string(),
            new_destination_station: "New Destination Station".to_string(),
            old_destination_system: "Old Destination System".to_string(),
            old_destination_station: "Old Destination Station".to_string(),
        };
        
        let json = serde_json::to_string(&event).expect("Failed to serialize MissionRedirectedEvent");
        let expected_json = r#"{"timestamp":"2025-01-13T18:05:28Z","MissionID":123456789,"Name":"Mission_Name","LocalisedName":"Localised Mission Name","NewDestinationSystem":"New Destination System","NewDestinationStation":"New Destination Station","OldDestinationSystem":"Old Destination System","OldDestinationStation":"Old Destination Station"}"#;
        assert_eq!(json, expected_json);
    }
}