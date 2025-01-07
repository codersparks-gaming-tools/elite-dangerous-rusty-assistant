use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use serde_with::serde_as;

/// Gives details about the current reputation with the "superpowers"
///
/// Example
/// ```json
/// {
///   "timestamp": "2025-01-04T19:27:09Z",
///   "event": "Reputation",
///   "Empire": 19.085400,
///   "Federation": 12.638300,
///   "Independent": 0.000000,
///   "Alliance": 10.122000
/// }
///
#[serde_as]
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct ReputationEvent {
    /// Timestamp of the event
    #[serde_as(as = "chrono::DateTime<chrono::Utc>")]
    #[serde(rename = "timestamp")]
    pub timestamp: NaiveDateTime,
    /// The current reputation with the empire faction
    pub empire: f32,
    /// The current reputation with the federation faction
    pub federation: f32,
    /// The current reputation with the independent faction
    pub independent: f32,
    /// The current reputation with the alliance faction
    pub alliance: f32
}


#[cfg(test)]
mod tests {
    use crate::events::reputation::ReputationEvent;
    use crate::test_helper::serde_helpers::create_timestamp;

    #[test]
    fn test_deserialise_reputation_event() {
        let timestamp_str = "2025-01-04T19:27:09Z";
        let timestamp = create_timestamp(timestamp_str);
        let json = format!(r#"{{ "timestamp":"{timestamp_str}", "event":"Reputation", "Empire":19.085400, "Federation":12.638300, "Independent":0.000000, "Alliance":10.122000 }}"#);

        let event: ReputationEvent = serde_json::from_str(&json).unwrap();
        assert_eq!(event.empire, 19.085400);
        assert_eq!(event.federation, 12.638300);
        assert_eq!(event.independent, 0.00000000);
        assert_eq!(event.alliance, 10.122000);
        assert_eq!(event.timestamp, timestamp);
    }
}