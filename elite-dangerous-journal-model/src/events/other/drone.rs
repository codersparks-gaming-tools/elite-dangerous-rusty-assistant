use serde::{Deserialize, Serialize};
use crate::events::common::EventMeta;


/// The type of drone/limpet
#[allow(missing_docs)]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all="PascalCase")]
pub enum DroneType {
    #[serde(rename="Hatchbreaker")]
    HatchBreaker,
    FuelTransfer,
    Collection,
    Prospector,
    Repair,
    Research,
    Docontamination,
}

/// Event emited when a drone/limpet is launched
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct LaunchDroneEvent {
    
    /// The event meta data
    #[serde(flatten)]
    pub event_meta: EventMeta,
    
    /// The type of drone/limpet launched
    #[serde(rename="Type")]
    pub drone_type: DroneType
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_helper::serde_helpers::create_timestamp;

    #[test]
    fn test_launch_drone_event_deserialization() {
        let timestamp_str = "2023-10-01T12:34:56Z";
        let json_data = format!(r#"{{
            "timestamp": "{timestamp_str}",
            "Type": "Hatchbreaker"
        }}"#);
        let timestamp = create_timestamp(timestamp_str);

        let deserialized_event: LaunchDroneEvent = serde_json::from_str(&json_data).unwrap();

        assert_eq!(deserialized_event.event_meta.timestamp, timestamp);
        assert_eq!(deserialized_event.drone_type, DroneType::HatchBreaker);
    }

}