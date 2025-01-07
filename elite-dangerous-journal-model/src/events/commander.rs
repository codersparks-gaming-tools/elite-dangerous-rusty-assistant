use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use serde_with::serde_as;

/// The commander event give the details of the loaded commander in the game
///
/// Example event:
/// ```json
/// {
///   "timestamp": "2025-01-04T19:27:09Z",
///   "event": "Commander",
///   "FID": "F00000000",
///   "Name": "ANON"
/// }
/// ```
#[serde_as]
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct CommanderEvent {
    /// The time of the event
    #[serde_as(as = "chrono::DateTime<chrono::Utc>")]
    pub timestamp: NaiveDateTime,
    /// The unique internal identifier of the commander
    #[serde(rename = "FID")]
    pub fid: String,
    /// The commanders name
    #[serde(rename = "Name")]
    pub name: String,
}

#[cfg(test)]
mod tests {
    use crate::events::commander::CommanderEvent;
    use crate::test_helper::serde_helpers::create_timestamp;

    #[test]
    fn test_deserialize_commander_event() {

        let timestamp_str = "2025-01-04T19:27:09Z";
        let timestamp = create_timestamp(timestamp_str);
        let json = format!(r#"{{ "timestamp":"{timestamp_str}", "event":"Commander", "FID":"F00000000", "Name":"ANON" }}"#);

        let event: CommanderEvent = serde_json::from_str(&json).unwrap();

        assert_eq!(timestamp, event.timestamp);
        assert_eq!(event.fid, String::from("F00000000"));
        assert_eq!(event.name, "ANON");
    }
}
