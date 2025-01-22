use serde::{Deserialize, Serialize};
use crate::events::common::{EventMeta, LocalisedValue};

/// The cargo item in the inventory
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(from = "CargoItemSchema", into = "CargoItemSchema")]
pub struct CargoItem {
    /// A localised value of name
    pub name: LocalisedValue,
    /// The number of this items
    pub count: u32,
    /// How many of these are stolen
    pub stolen: u32,
    /// If applicable the mission id associated with this cargo item
    pub mission_id: Option<u32>,
}

impl From<CargoItemSchema> for CargoItem {
    fn from(cargo_item_schema: CargoItemSchema) -> Self {
        Self {
            name: LocalisedValue { value: cargo_item_schema.name, localised_value: cargo_item_schema.name_localized },
            count: cargo_item_schema.count,
            stolen: cargo_item_schema.stolen,
            mission_id: cargo_item_schema.mission_id,
        }
    }
}

/// The raw schema for the cargo item
#[allow(missing_docs)]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct CargoItemSchema {
    pub name: String,
    #[serde(rename = "Name_Localised")]
    pub name_localized: Option<String>,
    pub count: u32,
    pub stolen: u32,
    #[serde(rename = "MissionID")]
    pub mission_id: Option<u32>,
}

impl From<CargoItem> for CargoItemSchema {
    fn from(cargo_item: CargoItem) -> Self {
        Self {
            name: cargo_item.name.value,
            name_localized: cargo_item.name.localised_value,
            count: cargo_item.count,
            stolen: cargo_item.stolen,
            mission_id: cargo_item.mission_id,
        }
    }
}


/// The vessel the cargo event is for
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum CargoVessel {
    /// Vessel is the current ship
    Ship,
    /// Vessel is the current SRV
    SRV,
}

/// Cargo event
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct CargoEvent {

    /// The meta data for the event
    #[serde(flatten)]
    pub event_meta: EventMeta,

    /// The vessel the cargo event is for
    pub vessel: CargoVessel,

    /// The total count of items
    pub count: u32,

    /// The Optional inventory (only supplied when inventory changes other than count of item)
    pub inventory: Option<Vec<CargoItem>>,
}


#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;
    use crate::test_helper::serde_helpers::create_timestamp;

    #[test]
    fn test_cargo_event_deserialization() {

        let timestamp_str = "2023-10-25T12:30:00Z";
        let timestamp = create_timestamp(timestamp_str);
        let json_data = format!(r#"
        {{
            "timestamp": "{timestamp_str}",
            "Vessel": "Ship",
            "Count": 15,
            "Inventory": [
                {{
                    "Name": "Gold",
                    "Name_Localised": "Gold",
                    "Count": 10,
                    "Stolen": 0,
                    "MissionID": 12345
                }},
                {{
                    "Name": "Silver",
                    "Count": 5,
                    "Stolen": 1
                }}
            ]
        }}
        "#);

        let event: CargoEvent = serde_json::from_str(&json_data).expect("Failed to deserialize CargoEvent");

        // Check deserialized fields
        assert_eq!(event.event_meta.timestamp,timestamp);
        assert_eq!(event.vessel, CargoVessel::Ship);
        assert_eq!(event.count, 15);

        let inventory = event.inventory.expect("Inventory should not be None");
        assert_eq!(inventory.len(), 2);

        // First cargo item
        assert_eq!(inventory[0].name.value, "Gold");
        assert_eq!(inventory[0].name.localised_value.as_deref(), Some("Gold"));
        assert_eq!(inventory[0].count, 10);
        assert_eq!(inventory[0].stolen, 0);
        assert_eq!(inventory[0].mission_id, Some(12345));

        // Second cargo item
        assert_eq!(inventory[1].name.value, "Silver");
        assert_eq!(inventory[1].name.localised_value, None);
        assert_eq!(inventory[1].count, 5);
        assert_eq!(inventory[1].stolen, 1);
        assert_eq!(inventory[1].mission_id, None);
    }


    #[test]
    fn test_cargo_event_deserialization_no_inventory() {
        let timestamp_str = "2023-10-25T12:30:00Z";
        let timestamp = create_timestamp(timestamp_str);
        let json_data = format!(r#"
        {{
            "timestamp": "{timestamp_str}",
            "Vessel": "SRV",
            "Count": 0
        }}
        "#);

        let event: CargoEvent = serde_json::from_str(&json_data).expect("Failed to deserialize CargoEvent");

        // Check deserialized fields
        assert_eq!(event.event_meta.timestamp, timestamp);
        assert_eq!(event.vessel, CargoVessel::SRV);
        assert_eq!(event.count, 0);
        assert!(event.inventory.is_none());
    }
}