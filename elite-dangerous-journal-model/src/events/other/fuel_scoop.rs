
use serde::{Deserialize, Serialize};
use crate::events::common::EventMeta;

/// Emitted when scooping fuel from a star
///
/// Example:
/// ```json
/// {
///   "timestamp": "2025-01-11T20:01:54Z",
///   "event": "FuelScoop",
///   "Scooped": 0.082209,
///   "Total": 32.000000
/// }
/// ```
///
/// ReadTheDocs: https://elite-journal.readthedocs.io/en/latest/Other%20Events/#fuelscoop
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct FuelScoopEvent {

    /// The event meta data
    #[serde(flatten)]
    pub event_meta: EventMeta,

    /// The amount of fuel scooped
    pub scooped: f32,
    /// The total fuel after scooping
    pub total: f32,
}
#[cfg(test)]
mod tests {
    use crate::events::other::fuel_scoop::FuelScoopEvent;
    use crate::test_helper::serde_helpers::create_timestamp;


    #[test]
    fn test_fuel_scoop_event() {
        let timestamp_str = "2025-01-11T20:01:54Z";
        let timestamp = create_timestamp(timestamp_str);
        let json = format!(r#"{{ "timestamp":"{timestamp_str}", "event":"FuelScoop", "Scooped":0.082209, "Total":32.000000 }}"#);

        let event: FuelScoopEvent = serde_json::from_str(&json).unwrap();

        assert_eq!(event.total, 32.000000);
        assert_eq!(event.scooped, 0.082209);
        assert_eq!(timestamp, event.event_meta.timestamp);



    }
}