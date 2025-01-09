
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use serde_with::serde_as;
use crate::events::EventMeta;

/// The current state of the relation ship of the commander with this engineer
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub enum EngineerProgress {
    Known,
    Unlocked,
    Invited
}

/// An enum to represent the current rank of the relationship with the engineer
///
/// We use the serde_repr to allow this to be represented as a number, for details see:
///  https://serde.rs/enum-number.html
#[derive(Serialize_repr, Deserialize_repr, Debug, PartialEq, Clone)]
#[repr(u8)]
pub enum EngineerRank {
    One = 1,
    Two = 2,
    Three = 3,
    Four = 4,
    Five = 5,
}

/// The engineer event contains a list of engineers, this struct represents the engineer info
///
/// Example:
/// ```json
///     {
///       "Engineer": "Felicity Farseer",
///       "EngineerID": 300100,
///       "Progress": "Unlocked",
///       "RankProgress": 14,
///       "Rank": 4
///     },
///
/// **Note**: `Rank` and `RankProgress` are both only present when progress is unlocked, currently this is not validated
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct Engineer {
    pub engineer: String,
    #[serde(rename = "EngineerID")]
    pub engineer_id: u32,
    pub progress: EngineerProgress,
    pub rank_progress: Option<u32>,
    pub rank: Option<EngineerRank>,
}

/// An event that represents the current relationship status with the engineers
///
/// Example:
/// ```json
/// {
///   "timestamp": "2025-01-04T19:27:09Z",
///   "event": "EngineerProgress",
///   "Engineers": [
///     {
///       "Engineer": "Felicity Farseer",
///       "EngineerID": 300100,
///       "Progress": "Unlocked",
///       "RankProgress": 14,
///       "Rank": 4
///     },
///     {
///       "Engineer": "Eleanor Bresa",
///       "EngineerID": 400011,
///       "Progress": "Known"
///     }
///   ]
/// }
/// ```
#[serde_as]
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct EngineerProgressEvent {
    /// The event metadata
    #[serde(flatten)]
    pub event_meta: EventMeta,
    /// List of engineers
    pub engineers: Vec<Engineer>,
}

#[cfg(test)]
mod tests {
    use chrono::NaiveDateTime;
    use crate::events::engineer::{Engineer, EngineerProgressEvent, EngineerProgress, EngineerRank};
    use crate::test_helper::serde_helpers::create_timestamp;

    #[test]
    fn test_deserialize_single_engineer_unlocked() {

        let json = r#"{ "Engineer":"Felicity Farseer", "EngineerID":300100, "Progress":"Unlocked", "RankProgress":14, "Rank":4 }"#;

        let engineer : Engineer = serde_json::from_str(&json).unwrap();

        assert_eq!(engineer.engineer, "Felicity Farseer");
        assert_eq!(engineer.engineer_id, 300100u32);
        assert_eq!(engineer.progress, EngineerProgress::Unlocked);
        assert_eq!(engineer.rank_progress, Some(14u32));
        assert_eq!(engineer.rank, Some(EngineerRank::Four));
    }

    #[test]
    fn test_deserialize_single_engineer_known() {
        let json = r#"{ "Engineer":"Eleanor Bresa", "EngineerID":400011, "Progress":"Known" }"#;

        let engineer: Engineer = serde_json::from_str(&json).unwrap();

        assert_eq!(engineer.engineer, "Eleanor Bresa");
        assert_eq!(engineer.engineer_id, 400011u32);
        assert_eq!(engineer.progress, EngineerProgress::Known);
        assert_eq!(engineer.rank_progress, None);
        assert_eq!(engineer.rank, None);
    }

    #[test]
    fn deserialize_event_multiple_engineers() {
        let timestamp_str = "2025-01-04T19:27:09Z";
        let timestamp: NaiveDateTime = create_timestamp(timestamp_str);
        let json = format!(r#"{{ "timestamp":"{timestamp_str}", "event":"EngineerProgress", "Engineers":[ {{ "Engineer":"Felicity Farseer", "EngineerID":300100, "Progress":"Unlocked", "RankProgress":14, "Rank":4 }}, {{ "Engineer":"Eleanor Bresa", "EngineerID":400011, "Progress":"Known" }} ] }}"#);

        let event: EngineerProgressEvent = serde_json::from_str(&json).unwrap();

        let engineer1 = Engineer {
            engineer: "Felicity Farseer".to_string(),
            engineer_id: 300100u32,
            progress: EngineerProgress::Unlocked,
            rank_progress: Some(14u32),
            rank: Some(EngineerRank::Four),
        };

        let engineer2 = Engineer {
            engineer: "Eleanor Bresa".to_string(),
            engineer_id: 400011u32,
            progress: EngineerProgress::Known,
            rank_progress: None,
            rank: None,
        };

        assert_eq!(timestamp, event.event_meta.timestamp);
        assert_eq!(event.engineers.len(), 2);
        assert!(event.engineers.contains(&engineer1));
        assert!(event.engineers.contains(&engineer2));
    }
}