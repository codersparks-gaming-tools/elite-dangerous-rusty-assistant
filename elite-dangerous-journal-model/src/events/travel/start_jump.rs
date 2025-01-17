use serde::{Deserialize, Serialize};
use crate::events::common::EventMeta;

/// Optional fields when type is hyperspace
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all="PascalCase")]
pub struct HypeerSpaceData {
    /// The name of a star system
    star_system: String,
    /// The type of the star
    star_class: String,
    /// The system address
    pub system_address: u64,

}

/// Jump type is an enum to show if it is HyperSpace or SuperCruise
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(tag="JumpType")]
pub enum JumpType {
    /// The jump type is inter system
    #[serde(rename="Hyperspace")]
    HyperSpace(HypeerSpaceData),
    /// The jump type is intra system
    #[serde(rename="Supercruise")]
    SuperCruise,
}

/// The actual jump event
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all="PascalCase")]
pub struct StartJumpEvent {
    /// The event meta data
    #[serde(flatten)]
    pub event_meta: EventMeta,
    /// The jump type
    #[serde(flatten)]
    pub jump_type: JumpType,

    /// Is it a tzxi journey
    pub taxi: bool

}


#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;
    use crate::test_helper::serde_helpers::create_timestamp;

    #[test]
    fn test_deserialize_start_jump_event_hyperspace() {
        let timestamp_str = "2025-01-04T19:49:15Z";
        let timestamp = create_timestamp(timestamp_str);
        let json_data = format!(r#"
        {{
            "timestamp": "{timestamp_str}",
            "JumpType": "Hyperspace",
            "StarSystem": "Sol",
            "StarClass": "G",
            "SystemAddress": 123456789,
            "Taxi": false
        }}
        "#);

        let result: StartJumpEvent = serde_json::from_str(&json_data).unwrap();

        assert_eq!(result.event_meta.timestamp, timestamp);
        assert_eq!(result.jump_type, JumpType::HyperSpace(HypeerSpaceData {
            star_system: "Sol".to_string(),
            star_class: "G".to_string(),
            system_address: 123456789
        }));
        assert_eq!(result.taxi, false);
    }

    #[test]
    fn test_deserialize_start_jump_event_supercruise() {
        let timestamp_str = "2025-01-04T19:49:15Z";
        let timestamp = create_timestamp(timestamp_str);
        let json_data = format!(r#"
        {{
            "timestamp": "{timestamp_str}",
            "JumpType": "Supercruise",
            "Taxi": true
        }}
        "#);

        let result: StartJumpEvent = serde_json::from_str(&json_data).unwrap();

        assert_eq!(result.event_meta.timestamp, timestamp);
        assert_eq!(result.jump_type, JumpType::SuperCruise);
        assert_eq!(result.taxi, true);
    }
}

