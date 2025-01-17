use serde_with::serde_as;
use serde::{Deserialize, Serialize};
use chrono::NaiveDateTime;

/// A common struct that is present in all events
#[serde_as]
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct EventMeta {
    /// Timestamp the event was received
    #[serde_as(as="chrono::DateTime<chrono::Utc>")]
    #[serde(rename = "timestamp")]
    pub timestamp: NaiveDateTime,
}


/// Meta data about a ship
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct ShipMeta {
    /// The ship being described (type)
    pub ship: String,
    /// The ID of the ship being described
    #[serde(rename = "ShipID")]
    pub ship_id: u8,
    /// The name of the ship
    pub ship_name: String,
    /// The in-game identity (e.g. "NCC-1701")
    pub ship_ident: String
}

/// A representation for an empty event that just contains the evnnt meta
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct EmptyEvent {
    /// the basic event meta data object
    #[serde(flatten)]
    pub meta: EventMeta,
}