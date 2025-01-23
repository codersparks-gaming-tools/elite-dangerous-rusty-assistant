use serde::Deserialize;
use serde_with::serde_derive::Serialize;
use crate::events::common::EventMeta;

/// The reservoir replenished event
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct ReservoirReplenishedEvent {
    
    /// The event meta data
    #[serde(flatten)]
    pub event_meta: EventMeta,
    
    /// The main fuel supply
    pub fuel_main: f32,
    
    /// The reservoir supply
    pub fuel_reservoir: f32,
    
}


#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;
    use crate::test_helper::serde_helpers::create_timestamp;

    #[test]
    fn test_reservoir_replenished_event_round_trip()  {
        
        let timestamp_str = "2023-10-01T12:00:00Z";
        let timestamp = create_timestamp(timestamp_str);
        let event = ReservoirReplenishedEvent {
            event_meta: EventMeta {
                timestamp,
            },
            fuel_main: 50.0,
            fuel_reservoir: 10.0,
        };


        // Serialize the struct into JSON
        let json = serde_json::to_string(&event).expect("Failed to serialize ReservoirReplenishedEvent");
        println!("{:?}", json);
        assert!(json.contains("\"timestamp\":\"2023-10-01T12:00:00Z\""));
        assert!(json.contains("\"FuelMain\":50.0"));
        assert!(json.contains("\"FuelReservoir\":10.0"));

        // Deserialize the JSON back into the struct
        let deserialized_event: ReservoirReplenishedEvent =
            serde_json::from_str(&json).expect("Failed to deserialize ReservoirReplenishedEvent");

        // Check if the original struct and the deserialized struct are the same
        assert_eq!(event, deserialized_event);
    }
}