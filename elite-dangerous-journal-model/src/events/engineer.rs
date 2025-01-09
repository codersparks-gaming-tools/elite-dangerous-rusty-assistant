
use serde::{Deserialize, Deserializer, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use serde_with::serde_as;
use tracing::debug;
use crate::events::engineer::EngineerData::{Multiple, Single};
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
    pub progress: Option<EngineerProgress>,
    pub rank_progress: Option<u32>,
    pub rank: Option<EngineerRank>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub enum EngineerData {
    Single(Engineer),
    Multiple(Vec<Engineer>),
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
///
/// Apparently there are two forms of this event, the above with multiple engineers but also a single engineer:
///
/// ```json
/// {
///   "timestamp": "2025-01-06T20:07:22Z",
///   "event": "EngineerProgress",
///   "Engineer": "Marco Qwent",
///   "EngineerID": 300200,
///   "Progress": "Invited"
/// }
///```
///
#[serde_as]
#[derive(Serialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct EngineerProgressEvent {
    /// The event metadata
    #[serde(flatten)]
    pub event_meta: EventMeta,
    /// The engineer data
    #[serde(flatten)]
    pub data: EngineerData,
}

impl<'de> Deserialize<'de> for EngineerProgressEvent {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>
    {
        #[derive(Deserialize, Debug)]
        #[serde(rename_all = "PascalCase")]
        struct EngineerHelper {
            #[serde(flatten)]
            event_meta: EventMeta,
            engineer: Option<String>,
            #[serde(rename = "EngineerID")]
            engineer_id: Option<u32>,
            progress: Option<EngineerProgress>,
            rank_progress: Option<u32>,
            rank: Option<EngineerRank>,
            engineers: Option<Vec<Engineer>>,
        }

        let helper = EngineerHelper::deserialize(deserializer)?;

        debug!("EngineerHelper interim value: {:?}", helper);

        if helper.engineers.is_some() {
            if helper.engineer_id.is_some() || helper.rank_progress.is_some() || helper.rank.is_some() || helper.engineer.is_some() {
                Err(serde::de::Error::custom("Both engineers and single cannot be represented"))
            } else {
                Ok(Self{
                    event_meta: helper.event_meta,
                    data: Multiple(helper.engineers.unwrap().clone()),
                })
            }
        } else {
            // To be a valid engineer it has to have the three fields set
            if helper.engineer_id.is_some() && helper.engineer.is_some() {
                let engineer = Engineer {
                    engineer: helper.engineer.unwrap(),
                    engineer_id: helper.engineer_id.unwrap(),
                    progress: helper.progress,
                    rank_progress: helper.rank_progress,
                    rank: helper.rank,
                };
                Ok(Self{
                    event_meta: helper.event_meta,
                    data: Single(engineer)
                })
            } else {
                Err(serde::de::Error::custom("Engineers not present and one of engineer_id, engineer or progress is not supplied"))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use chrono::NaiveDateTime;
    use crate::events::engineer::{Engineer, EngineerProgressEvent, EngineerProgress, EngineerRank, EngineerData};
    use crate::test_helper::serde_helpers::create_timestamp;

    #[test]
    fn test_deserialize_single_engineer_unlocked() {

        let json = r#"{ "Engineer":"Felicity Farseer", "EngineerID":300100, "Progress":"Unlocked", "RankProgress":14, "Rank":4 }"#;

        let engineer : Engineer = serde_json::from_str(&json).unwrap();

        assert_eq!(engineer.engineer, "Felicity Farseer");
        assert_eq!(engineer.engineer_id, 300100u32);
        assert_eq!(engineer.progress, Some(EngineerProgress::Unlocked));
        assert_eq!(engineer.rank_progress, Some(14u32));
        assert_eq!(engineer.rank, Some(EngineerRank::Four));
    }

    #[test]
    fn test_deserialize_single_engineer_known() {
        let json = r#"{ "Engineer":"Eleanor Bresa", "EngineerID":400011, "Progress":"Known" }"#;

        let engineer: Engineer = serde_json::from_str(&json).unwrap();

        assert_eq!(engineer.engineer, "Eleanor Bresa");
        assert_eq!(engineer.engineer_id, 400011u32);
        assert_eq!(engineer.progress, Some(EngineerProgress::Known));
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
            progress: Some(EngineerProgress::Unlocked),
            rank_progress: Some(14u32),
            rank: Some(EngineerRank::Four),
        };

        let engineer2 = Engineer {
            engineer: "Eleanor Bresa".to_string(),
            engineer_id: 400011u32,
            progress: Some(EngineerProgress::Known),
            rank_progress: None,
            rank: None,
        };

        assert_eq!(timestamp, event.event_meta.timestamp);
        match event.data {
            EngineerData::Multiple (engineers) => {
                assert_eq!(engineers.len(), 2);
                assert!(engineers.contains(&engineer1));
                assert!(engineers.contains(&engineer2));
            }
            _ => panic!("Wrong type of data for engineer, expected multiple but got {:?}", event.data),
        }
    }

    #[test]
    pub fn deserialize_event_single_engineer() {
        let timestamp_str = "2025-01-04T19:27:09Z";
        let timestamp: NaiveDateTime = create_timestamp(timestamp_str);
        let json = format!(r#"{{ "timestamp":"{timestamp_str}", "event":"EngineerProgress", "Engineer":"Marco Qwent", "EngineerID":300200, "Progress":"Invited" }}"#);

        let event : EngineerProgressEvent = serde_json::from_str(&json).unwrap();

        assert_eq!(event.event_meta.timestamp, timestamp);
        match event.data {
            EngineerData::Single(engineer) => {
                assert_eq!(engineer.engineer, "Marco Qwent");
                assert_eq!(engineer.engineer_id, 300200u32);
                assert_eq!(engineer.progress, Some(EngineerProgress::Invited));
            }
            _ => panic!("Wrong type of data for engineer, expected single engineer but got {:?}", event.data),
        }
    }
}