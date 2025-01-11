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
/// Contains structs to represent music events
pub mod music;
/// For scan events
pub mod scan;

/// A common struct that is present in all events
#[serde_as]
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct EventMeta {
    /// Timestamp the event was received
    #[serde_as(as="chrono::DateTime<chrono::Utc>")]
    #[serde(rename = "timestamp")]
    pub timestamp: NaiveDateTime,
}
