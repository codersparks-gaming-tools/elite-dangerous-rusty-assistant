use serde::{Deserialize, Serialize};
use crate::events::common::{EventMeta, LocalisedValue};

/// Event raised when fmission is abandoned
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(from = "MissionAbandonedEventSchema", into = "MissionAbandonedEventSchema")]
pub struct MissionAbandonedEvent {
    /// Event meta data
    pub event_meta: EventMeta,
    
    /// The name of the mission
    pub name: LocalisedValue,
    
    /// The mission id
    pub mission_id: u64,
}

impl From<MissionAbandonedEventSchema> for MissionAbandonedEvent {
    fn from(value: MissionAbandonedEventSchema) -> Self {
        
        let name = LocalisedValue::new(value.name, Some(value.localised_name));
        MissionAbandonedEvent {
            event_meta: value.event_meta,
            name,
            mission_id: value.mission_id,
        }
    }
}

/// As this event has localised value we need a schema
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct MissionAbandonedEventSchema {
    /// The event meta data
    #[serde(flatten)]
    pub event_meta: EventMeta,
    
    /// The name of the mission
    pub name: String,
    
    /// The localised name of the mission
    pub localised_name: String,
    
    /// The mission id
    #[serde(rename = "MissionID")]
    pub mission_id: u64,
}

impl From<MissionAbandonedEvent> for MissionAbandonedEventSchema {
    fn from(value: MissionAbandonedEvent) -> Self {
        Self {
            event_meta: value.event_meta,
            name: value.name.value,
            localised_name: value.name.localised_value.unwrap_or_default(),
            mission_id: value.mission_id,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_helper::serde_helpers::create_timestamp;
    
    #[test]
    fn test_deserialize_mission_abandoned_event() {
        let timestamp_str = "2025-01-13T18:05:28Z";
        let timestamp = create_timestamp(timestamp_str);
        let json = r#"{"timestamp":"2025-01-13T18:05:28Z","event":"MissionAbandoned","Name":"Mission_Name","LocalisedName":"Localised Mission Name","MissionID":123456789}"#;
        let event: MissionAbandonedEvent = serde_json::from_str(&json).expect("Failed to deserialize MissionAbandonedEvent");
        assert_eq!(event.event_meta.timestamp, timestamp);
        assert_eq!(event.name.value, "Mission_Name");
        assert_eq!(event.name.localised_value, Some("Localised Mission Name".to_string()));
        assert_eq!(event.mission_id, 123456789);
    }
    

    #[test]
    fn test_serialize_mission_abandoned_event() {
        let event = MissionAbandonedEvent {
            event_meta: EventMeta {
                timestamp: create_timestamp("2025-01-13T18:05:28Z"),
            },
            name: LocalisedValue::new("Mission_Name".to_string(), Some("Localised Mission Name".to_string())),
            mission_id: 123456789,
        };

        let json = serde_json::to_string(&event).expect("Failed to serialize MissionAbandonedEvent");
        let expected_json = r#"{"timestamp":"2025-01-13T18:05:28Z","Name":"Mission_Name","LocalisedName":"Localised Mission Name","MissionID":123456789}"#;
        assert_eq!(json, expected_json);
    }
}