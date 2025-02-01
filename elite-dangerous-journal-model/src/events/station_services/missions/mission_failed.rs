use serde::{Deserialize, Serialize};
use crate::events::common::{EventMeta, LocalisedValue};

/// The event raised for mission failed
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(from = "MissionFailedEventSchema", into = "MissionFailedEventSchema")]
pub struct MissionFailedEvent {
    /// The event meta data
    pub event_meta: EventMeta,

    /// TGhe name of the mission
    pub name: LocalisedValue,

    /// The mission ID
    pub mission_id: u64,
    
    /// Optional fine
    pub fine: Option<u64>,
}

impl From<MissionFailedEventSchema> for MissionFailedEvent {
    
    /// Convert from MissionFailedEventSchema
    fn from(value: MissionFailedEventSchema) -> Self {
        Self {
            event_meta: value.event_meta,
            name: LocalisedValue::new(value.name, Some(value.localised_name)),
            mission_id: value.mission_id,
            fine: value.fine,
        }
    }
}

/// Schema for mission failed event as it contains a localised value
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct MissionFailedEventSchema {
    
    /// The event meta data
    #[serde(flatten)]
    pub event_meta: EventMeta,
    
    /// TGhe name of the mission
    pub name: String,
    
    /// The localised name of the mission
    pub localised_name: String,
    
    /// The mission ID
    #[serde(rename = "MissionID")]
    pub mission_id: u64,
    
    /// Optional fine
    pub fine: Option<u64>,
    
}

impl From<MissionFailedEvent> for MissionFailedEventSchema {
    fn from(value: MissionFailedEvent) -> Self {
        Self {
            event_meta: value.event_meta,
            name: value.name.value,
            localised_name: value.name.localised_value.unwrap(),
            mission_id: value.mission_id,
            fine: value.fine,
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_helper::serde_helpers::create_timestamp;

    #[test]
    fn test_deserialize_mission_failed_event() {
        let json_data = r#"
        {
            "timestamp": "2025-01-24T22:32:44Z",
            "event": "MissionFailed",
            "Name": "Mission_Delivery_Boom_name",
            "LocalisedName": "Boom time delivery of 72 units of Silver",
            "MissionID": 1000102950
        }
        "#;

        let mission_event: MissionFailedEventSchema = serde_json::from_str(json_data).expect("Failed to deserialize JSON");

        assert_eq!(mission_event.event_meta.timestamp, create_timestamp("2025-01-24T22:32:44Z"));
        assert_eq!(mission_event.name, "Mission_Delivery_Boom_name");
        assert_eq!(mission_event.localised_name, "Boom time delivery of 72 units of Silver");
        assert_eq!(mission_event.mission_id, 1000102950);
    }
}