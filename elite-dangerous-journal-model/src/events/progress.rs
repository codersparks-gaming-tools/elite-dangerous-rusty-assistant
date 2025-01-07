use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use serde_with::serde_as;

/// Gives details about the current percent progress towards next ranks for the commander
///
/// Example:
/// ```json
/// {
///   "timestamp": "2025-01-04T19:27:09Z",
///   "event": "Progress",
///   "Combat": 22,
///   "Trade": 15,
///   "Explore": 60,
///   "Soldier": 0,
///   "Exobiologist": 0,
///   "Empire": 11,
///   "Federation": 2,
///   "CQC": 0
/// }
/// ```
#[serde_as]
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct ProgressEvent {
    /// Timestamp the event was received
    #[serde_as(as="chrono::DateTime<chrono::Utc>")]
    #[serde(rename = "timestamp")]
    pub timestamp: NaiveDateTime,
    /// The current combat rating of the commander
    pub combat: u8,
    /// The current trade rating of the commander - Trader rank in wiki
    pub trade: u8,
    /// The current exploration rating of the commander - Explorer rank in wiki
    pub explore: u8,
    /// The current soldier rating of the commander - Assumed that this is mercenary rank in wiki
    #[serde(rename = "Soldier")]
    pub soldier: u8,
    /// The current exobiologist rating of the commander
    pub exobiologist: u8,
    /// The current empire rating of the commander - Imperial navy in the wiki
    pub empire: u8,
    /// The current federation rating of the commander - Federal navy in the wiki
    pub federation: u8,
    /// The current close quarter combat (PVP) ranking of the commander
    #[serde(rename = "CQC")]
    pub cqc: u8,
}

#[cfg(test)]
mod tests {
    use crate::events::progress::ProgressEvent;
    use crate::test_helper::serde_helpers::create_timestamp;

    #[test]
    fn test_deserialize_rank_event() {
        let timestamp_str = "2025-01-04T19:27:09Z";
        let timestamp = create_timestamp(timestamp_str);
        let json = format!(r#"{{ "timestamp":"{timestamp_str}", "event":"Progress", "Combat":22, "Trade":15, "Explore":60, "Soldier":0, "Exobiologist":0, "Empire":11, "Federation":2, "CQC":0 }}"#);

        let event: ProgressEvent = serde_json::from_str(&json).unwrap();

        assert_eq!(event.combat, 22);
        assert_eq!(event.trade, 15);
        assert_eq!(event.explore, 60);
        assert_eq!(event.soldier, 0);
        assert_eq!(event.federation, 2);
        assert_eq!(event.cqc, 0);
        assert_eq!(event.empire, 11);
        assert_eq!(event.exobiologist, 0);
        assert_eq!(timestamp, event.timestamp);

    }
}