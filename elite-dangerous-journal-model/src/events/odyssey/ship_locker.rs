use serde::Deserialize;
use serde_with::serde_derive::Serialize;
use crate::events::common::EventMeta;
use crate::events::odyssey::common::Item;

/// Struct to represent data in a ShipLocker event
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct ShipLockerEvent {
    /// The common event metadata
    #[serde(flatten)]
    pub event_meta: EventMeta,

    /// List of items in the ShipLocker
    pub items: Option<Vec<Item>>,

    /// List of components in the ship locker
    pub components: Option<Vec<Item>>,

    /// List of consumables in the ship locker
    pub consumables: Option<Vec<Item>>,

    /// List of data in the ship locker
    pub data: Option<Vec<Item>>,
}

#[cfg(test)]
mod tests {
    use crate::events::odyssey::common::Item;
    use crate::events::odyssey::ship_locker::ShipLockerEvent;
    use crate::test_helper::serde_helpers::create_timestamp;

    #[test]
    fn test_ship_locker_event_no_fields() {

        let timestamp_str = "2025-01-11T22:00:00Z";
        let timestamp = create_timestamp(timestamp_str);
        let json = format!(r#"{{ "timestamp":"{timestamp_str}", "event":"ShipLocker" }}"#);

        let event: ShipLockerEvent = serde_json::from_str(&json).unwrap();

        assert_eq!(event.event_meta.timestamp, timestamp);
        assert_eq!(event.data, None);
        assert_eq!(event.components, None);
        assert_eq!(event.consumables, None);
        assert_eq!(event.items, None);
    }

    #[test]
    fn test_ship_locker_event_with_empty_fields() {
        let timestamp_str = "2025-01-11T21:56:34Z";
        let timestamp = create_timestamp(timestamp_str);
        let json = format!(r#"{{ "timestamp":"{timestamp_str}", "event":"ShipLocker", "Items":[  ], "Components":[  ], "Consumables":[  ], "Data":[  ] }}"#);

        let event: ShipLockerEvent = serde_json::from_str(&json).unwrap();

        assert_eq!(event.event_meta.timestamp, timestamp);
        assert_eq!(event.data, Some(vec![]));
        assert_eq!(event.components, Some(vec![]));
        assert_eq!(event.consumables, Some(vec![]));
        assert_eq!(event.items, Some(vec![]));
    }

    // TODO: Replace manually created items etc with event from log
    #[test]
    fn test_ship_locker_event_with_items() {
        let timestamp_str = "2025-01-11T21:56:34Z";
        let timestamp = create_timestamp(timestamp_str);
        let json = format!(r#"{{ "timestamp":"{timestamp_str}", "event":"ShipLocker", "Items":[ {{ "Name":"item1", "OwnerID": "F1111111", "MissionID":111111, "Count":3 }}, {{ "Name":"item2", "OwnerID": "F1111111", "MissionID":111111, "Count":2 }} ], "Components":[ {{ "Name":"component1", "OwnerID": "F1111111", "MissionID":111111, "Count":2 }} ], "Consumables":[  ], "Data":[  ] }}"#);

        let event: ShipLockerEvent = serde_json::from_str(&json).unwrap();
        println!("{:#?}", event);

        let item1 = Item{
            name: "item1".to_string(),
            owner_id: "F1111111".to_string(),
            mission_id: Some(111111),
            count: 3,
        };
        let item2 = Item{
            name: "item2".to_string(),
            owner_id: "F1111111".to_string(),
            mission_id: Some(111111),
            count: 2,
        };
        let component1 = Item{
            name: "component1".to_string(),
            owner_id: "F1111111".to_string(),
            mission_id: Some(111111),
            count: 2,
        };

        assert_eq!(event.event_meta.timestamp, timestamp);
        assert_eq!(event.items, Some(vec![item1, item2]));
        assert_eq!(event.data, Some(vec![]));
        assert_eq!(event.components, Some(vec![component1]));
        assert_eq!(event.consumables, Some(vec![]));
    }

}