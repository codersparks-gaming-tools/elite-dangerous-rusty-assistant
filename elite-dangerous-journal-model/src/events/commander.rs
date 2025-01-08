use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use crate::events::{EventMeta};

/// Struct to represent the commander details, used in multiple events
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct Commander {
    /// The unique internal identifier of the commander
    #[serde(rename = "FID")]
    pub fid: String,
    /// The commanders name
    #[serde(rename = "Name")]
    pub name: String,
}

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
    /// The event metadata
    #[serde(flatten)]
    pub event_meta: EventMeta,

    #[serde(flatten)]
    pub commander: Commander,
}

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
pub struct CommanderProgressEvent {

    /// The event metadata
    #[serde(flatten)]
    pub event_meta: EventMeta,

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
pub struct CommanderReputationEvent {
    /// The event metadata
    #[serde(flatten)]
    pub event_meta: EventMeta,
    /// The current reputation with the empire faction
    pub empire: f32,
    /// The current reputation with the federation faction
    pub federation: f32,
    /// The current reputation with the independent faction
    pub independent: f32,
    /// The current reputation with the alliance faction
    pub alliance: f32
}

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
pub struct CommanderRankEvent {
    /// The event metadata
    #[serde(flatten)]
    pub event_meta: EventMeta,
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
    use crate::test_helper::serde_helpers::create_timestamp;
    use crate::events::commander::{CommanderEvent, CommanderProgressEvent, CommanderRankEvent, CommanderReputationEvent};
    use crate::events::EventType;
    #[test]
    fn test_deserialize_progress_event() {
        let timestamp_str = "2025-01-04T19:27:09Z";
        let timestamp = create_timestamp(timestamp_str);
        let json = format!(r#"{{ "timestamp":"{timestamp_str}", "event":"Progress", "Combat":22, "Trade":15, "Explore":60, "Soldier":0, "Exobiologist":0, "Empire":11, "Federation":2, "CQC":0 }}"#);

        let event: CommanderProgressEvent = serde_json::from_str(&json).unwrap();

        assert_eq!(event.combat, 22);
        assert_eq!(event.trade, 15);
        assert_eq!(event.explore, 60);
        assert_eq!(event.soldier, 0);
        assert_eq!(event.federation, 2);
        assert_eq!(event.cqc, 0);
        assert_eq!(event.empire, 11);
        assert_eq!(event.exobiologist, 0);
        assert_eq!(timestamp, event.event_meta.timestamp);
        assert_eq!(event.event_meta.event, EventType::CommanderProgress);
    }


    #[test]
    fn test_deserialize_commander_event() {
        let timestamp_str = "2025-01-04T19:27:09Z";
        let timestamp = create_timestamp(timestamp_str);
        let json = format!(r#"{{ "timestamp":"{timestamp_str}", "event":"Commander", "FID":"F00000000", "Name":"ANON" }}"#);

        let event: CommanderEvent = serde_json::from_str(&json).unwrap();

        assert_eq!(timestamp, event.event_meta.timestamp);
        assert_eq!(event.event_meta.event, EventType::Commander);
        assert_eq!(event.commander.fid, String::from("F00000000"));
        assert_eq!(event.commander.name, "ANON");
    }

    #[test]
    fn test_deserialise_reputation_event() {
        let timestamp_str = "2025-01-04T19:27:09Z";
        let timestamp = create_timestamp(timestamp_str);
        let json = format!(r#"{{ "timestamp":"{timestamp_str}", "event":"Reputation", "Empire":19.085400, "Federation":12.638300, "Independent":0.000000, "Alliance":10.122000 }}"#);

        let event: CommanderReputationEvent = serde_json::from_str(&json).unwrap();
        assert_eq!(event.empire, 19.085400);
        assert_eq!(event.federation, 12.638300);
        assert_eq!(event.independent, 0.00000000);
        assert_eq!(event.alliance, 10.122000);
        assert_eq!(event.event_meta.timestamp, timestamp);
        assert_eq!(event.event_meta.event, EventType::CommanderReputation);
    }

    #[test]
    fn test_deserialize_rank_event() {
        let timestamp_str = "2025-01-04T19:27:09Z";
        let timestamp = create_timestamp(timestamp_str);
        let json = format!(r#"{{ "timestamp":"{timestamp_str}", "event":"Rank", "Combat":2, "Trade":6, "Explore":3, "Soldier":0, "Exobiologist":0, "Empire":5, "Federation":0, "CQC":0 }}"#);

        let event: CommanderRankEvent = serde_json::from_str(&json).unwrap();

        assert_eq!(event.combat, 2);
        assert_eq!(event.trade, 6);
        assert_eq!(event.explore, 3);
        assert_eq!(event.soldier, 0);
        assert_eq!(event.federation, 0);
        assert_eq!(event.cqc, 0);
        assert_eq!(event.empire, 5);
        assert_eq!(event.exobiologist, 0);
        assert_eq!(timestamp, event.event_meta.timestamp);
        assert_eq!(event.event_meta.event, EventType::CommanderRank);
    }
}