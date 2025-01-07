use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use serde_with::serde_as;

/// Gives details about the current ranks for the commander
///
/// Example:
/// ```json
/// {
///   "timestamp": "2025-01-04T19:27:09Z",
///   "event": "Rank",
///   "Combat": 2,
///   "Trade": 6,
///   "Explore": 3,
///   "Soldier": 0,
///   "Exobiologist": 0,
///   "Empire": 0,
///   "Federation": 0,
///   "CQC": 0
/// }
/// ```
/// The rating to ranks translation can be found at [the limited API docs](https://edcodex.info/?m=doc#f.11.1)
/// For explanation of the ranks see [Ranks wiki](https://elite-dangerous.fandom.com/wiki/Ranks)
#[serde_as]
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct RankEvent {
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
    use crate::events::rank::RankEvent;
    use crate::test_helper::serde_helpers::create_timestamp;

    #[test]
    fn test_deserialize_rank_event() {
        let timestamp_str = "2025-01-04T19:27:09Z";
        let timestamp = create_timestamp(timestamp_str);
        let json = format!(r#"{{ "timestamp":"{timestamp_str}", "event":"Rank", "Combat":2, "Trade":6, "Explore":3, "Soldier":0, "Exobiologist":0, "Empire":5, "Federation":0, "CQC":0 }}"#);

        let event: RankEvent = serde_json::from_str(&json).unwrap();

        assert_eq!(event.combat, 2);
        assert_eq!(event.trade, 6);
        assert_eq!(event.explore, 3);
        assert_eq!(event.soldier, 0);
        assert_eq!(event.federation, 0);
        assert_eq!(event.cqc, 0);
        assert_eq!(event.empire, 5);
        assert_eq!(event.exobiologist, 0);
        assert_eq!(timestamp, event.timestamp);

    }
}