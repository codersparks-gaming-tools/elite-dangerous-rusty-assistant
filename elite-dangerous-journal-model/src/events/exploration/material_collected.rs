use serde::{Deserialize, Serialize};
use crate::events::common::{EventMeta, LocalisedValue};

/// The category of material collected
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub enum MaterialCategory {
    /// Manufactured material
    Manufactured,
    /// Raw material
    Raw,
    /// Encoded data
    Encoded,
}

/// The material collected event
#[derive(Serialize, Deserialize,Debug, PartialEq, Clone)]
#[serde(from = "MaterialCollectedEventSchema", into = "MaterialCollectedEventSchema")]
pub struct MaterialCollectedEvent {
    /// The event meta data
    pub event_meta: EventMeta,

    /// The category of material collected
    pub category: MaterialCategory,

    /// The name of the material
    pub name: LocalisedValue,

    /// The number collected
    pub count: u64,

}

impl From<MaterialCollectedEventSchema> for MaterialCollectedEvent {
    fn from(value: MaterialCollectedEventSchema) -> Self {
        Self {
            event_meta: value.event_meta,
            category: value.category,
            name: LocalisedValue {
                value: value.name,
                localised_value: value.name_localised,
            },
            count: value.count,
        }
    }
}

/// The schema representation of the data
///
/// Example event:
/// ```json
///{
///   "timestamp": "2025-01-11T19:41:17Z",
///   "event": "MaterialCollected",
///   "Category": "Encoded",
///   "Name": "disruptedwakeechoes",
///   "Name_Localised": "Atypical Disrupted Wake Echoes",
///   "Count": 3
/// }
/// ```
#[derive(Serialize, Deserialize,Debug, PartialEq, Clone)]
#[serde(rename_all = "PascalCase")]
pub(crate) struct MaterialCollectedEventSchema {

    /// The event meta data
    #[serde(flatten)]
    pub event_meta: EventMeta,

    /// The category of material collected
    pub category: MaterialCategory,

    /// The name of the material
    pub name: String,

    /// The localised name of the material
    #[serde(rename = "Name_Localised")]
    pub name_localised: Option<String>,

    /// The number collected
    pub count: u64,
}

impl From<MaterialCollectedEvent> for MaterialCollectedEventSchema {
    fn from(value: MaterialCollectedEvent) -> Self {

        Self {
            event_meta: value.event_meta,
            category: value.category,
            name: value.name.value,
            name_localised: value.name.localised_value,
            count: value.count,
        }
    }
}



#[cfg(test)]
mod tests {
    use crate::events::exploration::material_collected::{MaterialCategory, MaterialCollectedEvent};
    use crate::test_helper::serde_helpers::create_timestamp;

    #[test]
    pub fn test_deserialize_material_collected() {
        let timestamp_str = "2025-01-11T19:28:36Z";
        let timestamp = create_timestamp(timestamp_str);
        let json = format!(r#"{{ "timestamp":"{timestamp_str}", "event":"MaterialCollected", "Category":"Manufactured", "Name":"imperialshielding", "Name_Localised":"Imperial Shielding", "Count":3 }}"#);


        let event : MaterialCollectedEvent = serde_json::from_str(&json).unwrap();

        assert_eq!(timestamp, event.event_meta.timestamp);
        assert_eq!(event.category, MaterialCategory::Manufactured);
        assert_eq!(event.name.value, "imperialshielding");
        assert_eq!(event.name.localised_value.unwrap(), "Imperial Shielding");
        assert_eq!(event.count, 3);
    }
}