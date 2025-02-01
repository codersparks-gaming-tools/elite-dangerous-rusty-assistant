
/// Common system structs
pub mod system;

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

/// A structure that allows a field that has both localised and non localised value as one
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct LocalisedValue {
    /// The non-localised value
    pub value: String,
    /// The localised value
    pub localised_value: Option<String>
}

impl LocalisedValue {
    
    /// Create a localised value
    pub fn new(value: String, localised_value: Option<String>) -> Self {
        Self { value, localised_value }
    }
    
    /// Create a localised value option
    pub fn new_optional(value: Option<String>, localised_value: Option<String>) -> Option<Self> {
        match value {
            None => None,
            Some(v) => {
                if let Some(lv) = localised_value {
                    Some(LocalisedValue { value: v, localised_value: Some(lv) })
                } else {
                    Some(LocalisedValue { value: v, localised_value: None })
                }
            }
        }
    }
    
    
    /// If the localise value is present it will return that value otherwise it returns the value
    pub fn get_value(&self) -> &str {

        if self.localised_value.is_some() {
            self.localised_value.as_ref().unwrap();
        }

        self.value.as_str()
    }
    
}

/// Allows getting the value and localised from a single call
pub fn deconstruct_localised_value(value: LocalisedValue) -> (String, Option<String>) {
    (String::from(value.get_value()), value.localised_value)
}

/// Allows getting the value and localised from an optional localised value from a single call
pub fn deconstruct_optional_localised_value(value: Option<LocalisedValue>) -> (Option<String>, Option<String>) {
    match value {
        None => (None, None),
        Some(v) => (Some(String::from(v.get_value())), v.localised_value)
    }
}