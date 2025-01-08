use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use serde_with::serde_as;

/// Contains model structs for Commander events
pub mod commander;
/// Contains model structs for Material events
pub mod material;
/// Contains structs for FSSSignalDiscoverd events
pub mod fss_signal_discovered;
/// Contains structs to represent engineer events
pub mod engineer;
/// Contains structs to represent game events
pub mod game;
/// Contains structs to represent ship events
pub mod ship;

// /// Enum to hold the different event types there are
// #[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
// #[serde(rename_all="PascalCase")]
// pub enum EventType {
//     Commander,
//     #[serde(rename = "Progress")]
//     CommanderProgress,
//     #[serde(rename = "Rank")]
//     CommanderRank,
//     #[serde(rename = "Reputation")]
//     CommanderReputation,
//     EngineerProgress,
//     #[serde(rename = "Fileheader")]
//     FileHeader,
//     FSSSignalDiscovered,
//     LoadGame,
//     Materials,
// }

#[serde_as]
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct EventMeta {
    /// Timestamp the event was received
    #[serde_as(as="chrono::DateTime<chrono::Utc>")]
    #[serde(rename = "timestamp")]
    pub timestamp: NaiveDateTime,
}
